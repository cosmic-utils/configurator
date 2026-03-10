use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
    iter::{self},
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{anyhow, bail};
use cosmic::widget::segmented_button::Entity;
use directories::BaseDirs;

use include_dir::include_dir;
use rust_schema2::RustSchemaRoot;

use crate::{
    app::{self, Dialog},
    config::Config,
    generic_value::Value,
    message::{ChangeMsg, PageMsg},
    node::{
        self, Node, NodeContainer,
        data_path::{DataPath, DataPathType},
        schema_at,
    },
    providers,
};

use configurator_utils::ConfigFormat;

#[derive(Debug)]
pub struct Page {
    pub appid: String,
    pub title: String,

    pub source_paths: Vec<PathBuf>,
    pub source_home_path: PathBuf,
    pub write_path: PathBuf,
    pub format: ConfigFormat,

    pub system_config: Value,
    pub user_config: Value,
    pub full_config: Value,

    pub schema_root: RustSchemaRoot,
    pub tree: NodeContainer,
    pub data_path: DataPath,
}

impl Page {
    // need &str for appid: https://github.com/tokio-rs/tracing/issues/1181
    #[instrument(skip(content))]
    fn from_str(appid: &str, content: &str) -> anyhow::Result<Self> {
        let json_value = json::Value::from_str(content)?;

        let Some(json_obj) = json_value.as_object() else {
            bail!("no obj")
        };

        let source_paths = {
            if let Some(json::Value::String(paths)) = json_obj.get("X_CONFIGURATOR_SOURCE_PATHS") {
                paths.split_terminator(';').map(PathBuf::from).collect()
            } else {
                vec![]
            }
        };

        let source_home_path = {
            if let Some(json::Value::String(path)) = json_obj.get("X_CONFIGURATOR_SOURCE_HOME_PATH")
            {
                let base_dirs = BaseDirs::new().unwrap();

                base_dirs.home_dir().join(path)
            } else {
                bail!("no X_CONFIGURATOR_SOURCE_HOME_PATH")
            }
        };

        let write_path = {
            if let Some(json::Value::String(path)) = json_obj.get("X_CONFIGURATOR_WRITE_PATH") {
                PathBuf::from(path)
            } else {
                source_home_path.clone()
            }
        };

        let format = {
            if let Some(json::Value::String(format)) = json_obj.get("X_CONFIGURATOR_FORMAT") {
                format
            } else {
                source_home_path
                    .extension()
                    .expect("no format defined")
                    .to_str()
                    .unwrap()
            }
        };

        let format = ConfigFormat::try_from(format)?;

        let mut system_config = Value::Empty;

        for path in &source_paths {
            system_config = system_config.merge(&providers::read_from_format(path, &format))
        }

        let user_config =
            Value::Empty.merge(&providers::read_from_format(&source_home_path, &format));

        let full_config = Value::Empty.merge(&system_config).merge(&user_config);

        info!("start generating node from schema");

        let data_path = DataPath::new();

        let schema_root = json::from_value(json_value)?;

        let mut tree = NodeContainer::from_schema_and_value(
            &schema_root,
            schema_root.resolve_schema(&schema_root.schema).unwrap(),
            &full_config,
        );

        tree.set_modified_from_value(&user_config);

        dbg!(&tree);

        let title = appid.split('.').next_back().unwrap().to_string();

        let page = Self {
            title,
            appid: appid.to_string(),
            system_config,
            user_config,
            full_config,
            data_path,
            source_paths,
            source_home_path,
            write_path,
            format,
            schema_root,
            tree,
        };

        Ok(page)
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    #[instrument(skip_all)]
    pub fn reload_config(&mut self) -> anyhow::Result<()> {
        info!("reload the config");

        self.user_config = Value::Empty.merge(&providers::read_from_format(
            &self.source_home_path,
            &self.format,
        ));

        debug!("user_config = {:#?}", self.user_config);

        debug!("system_config = {:#?}", self.system_config);

        self.full_config = Value::Empty
            .merge(&self.system_config)
            .merge(&self.user_config);

        debug!("full_config = {:#?}", self.full_config);

        Ok(())
    }

    #[instrument(skip_all)]
    pub fn reload_page(&mut self) -> anyhow::Result<()> {
        self.reload_config()?;

        self.tree = NodeContainer::from_schema_and_value(
            &self.schema_root,
            self.schema_root
                .resolve_schema(&self.schema_root.schema)
                .unwrap(),
            &self.full_config,
        );

        self.tree.set_modified_from_value(&self.user_config);

        self.data_path.sanitize_path(&self.tree);

        Ok(())
    }

    pub fn write(&self) -> anyhow::Result<()> {
        match self.tree.to_value() {
            Some(value) => {
                providers::write(&self.write_path, &self.format, value)?;
            }
            None => bail!("no value to write"),
        }

        Ok(())
    }
}

#[must_use]
pub enum Action {
    CreateDialog(Dialog),
    RemoveDialog,
    None,
}

impl Page {
    pub fn update(&mut self, message: PageMsg, page_id: Entity) -> Action {
        let action = Action::None;

        match message {
            PageMsg::SelectDataPath(pos) => {
                self.data_path.change_to(pos);
            }
            PageMsg::OpenDataPath(data_path_type) => {
                self.data_path.open(data_path_type);
            }

            PageMsg::ApplyDefault(data_path) => {
                let node = self.tree.get_at_mut(Box::new(data_path.iter())).unwrap();

                node.remove_value_rec();

                let schema = schema_at(&self.schema_root, &data_path).unwrap();

                // todo: get the default value
                // *node = NodeContainer::from_schema_and_value(&self.schema_root, schema, value);

                self.tree.set_modified(&data_path[..data_path.len() - 1]);
            }
            PageMsg::ChangeMsg(data_path, change_msg) => {
                debug!("{:?} {:?}", data_path, change_msg);

                let node = self.tree.get_at_mut(Box::new(data_path.iter())).unwrap();

                match change_msg {
                    ChangeMsg::ChangeBool(_) => todo!(),
                    ChangeMsg::ChangeString(value) => {
                        let node_string = node.node.unwrap_string_mut();
                        node_string.value = Some(value);
                    }
                    ChangeMsg::ChangeNumber(_) => todo!(),
                    ChangeMsg::ChangeEnum(_) => todo!(),
                    ChangeMsg::Remove(data) => match &mut node.node {
                        Node::Array(node_array) => {
                            node_array
                                .value
                                .as_mut()
                                .unwrap()
                                .remove(data.unwrap_indice());

                            for n in node_array.value.as_mut().unwrap() {
                                n.modified = true;
                            }
                        }
                        _ => panic!(),
                    },
                    ChangeMsg::AddNewNodeToObject(_) => todo!(),

                    ChangeMsg::AddNewNodeToArray => {
                        let node_array = node.node.unwrap_array_mut();

                        let array_schema = schema_at(&self.schema_root, &data_path).unwrap();

                        let array = array_schema.as_array().unwrap();

                        let template = array.template.as_ref().unwrap();
                        let template = self.schema_root.resolve_schema(template).unwrap();

                        dbg!(&template);

                        let mut new_node = NodeContainer::from_schema_and_value(
                            &self.schema_root,
                            template,
                            &Value::Empty,
                        )
                        .set_is_removable(true);

                        dbg!(&new_node);

                        new_node.modified = true;

                        match &mut node_array.value {
                            Some(values) => {
                                for n in &mut *values {
                                    n.modified = true;
                                }
                                values.push(new_node);
                            }
                            None => {
                                node_array.value = Some(vec![new_node]);
                            }
                        }
                    }
                    ChangeMsg::RenameKey { prev, new } => todo!(),
                }

                self.tree.set_modified(data_path.iter());

                self.data_path.sanitize_path(&self.tree);

                if self.tree.is_valid() {
                    self.write().unwrap();
                } else {
                    info!("tree is not valid")
                }
            }

            /*
            PageMsg::ChangeMsg(data_path, change_msg) => {
                let node = self.tree.get_at_mut(data_path.iter()).unwrap();

                match change_msg {
                    ChangeMsg::ApplyDefault => {
                        node.remove_value_rec();
                        node.apply_value(&node.default.clone().unwrap(), false)
                            .unwrap();

                        self.tree
                            .set_modified(data_path[..data_path.len() - 1].iter());
                    }
                    ChangeMsg::ChangeBool(value) => {
                        let node_bool = node.node.unwrap_bool_mut();
                        node_bool.value = Some(value);
                        self.tree.set_modified(data_path.iter());
                    }
                    ChangeMsg::ChangeString(value) => {
                        let node_string = node.node.unwrap_string_mut();
                        node_string.value = Some(value);
                        self.tree.set_modified(data_path.iter());
                    }
                    ChangeMsg::ChangeNumber(value) => {
                        let node_number = node.node.unwrap_number_mut();
                        node_number.value_string = value;

                        match node_number.try_parse_from_str(&node_number.value_string) {
                            Ok(v) => {
                                node_number.value = Some(v);
                            }
                            Err(_) => {
                                return Action::None;
                            }
                        }

                        self.tree.set_modified(data_path.iter());
                    }
                    ChangeMsg::ChangeEnum(value) => {
                        let node_enum = node.node.unwrap_enum_mut();
                        node_enum.value = Some(value);

                        node_enum.nodes[value].modified = true;
                        self.tree.set_modified(data_path.iter());
                    }
                    ChangeMsg::Remove(field) => {
                        match &mut node.node {
                            Node::Object(node_object) => {
                                node_object.nodes.shift_remove(field.unwrap_name_ref());

                                for n in node_object.nodes.values_mut() {
                                    n.modified = true;
                                }
                            }
                            Node::Array(node_array) => {
                                node_array
                                    .values
                                    .as_mut()
                                    .unwrap()
                                    .remove(field.unwrap_indice());

                                for n in node_array.values.as_mut().unwrap() {
                                    n.modified = true;
                                }
                            }
                            _ => panic!(),
                        }
                        // dbg!(&self.data_path);

                        self.tree.set_modified(data_path.iter());
                    }
                    ChangeMsg::AddNewNodeToObject(name) => {
                        let node_object = node.node.unwrap_object_mut();

                        if node_object.nodes.contains_key(&name) {
                            return Action::None;
                        }

                        let mut new_node = node_object.template().unwrap();

                        let default = new_node.default.clone().unwrap();
                        new_node.apply_value(&default, false).unwrap();

                        node_object.nodes.insert(name, new_node);

                        for n in node_object.nodes.values_mut() {
                            n.modified = true;
                        }

                        self.tree.set_modified(data_path.iter());

                        action = Action::RemoveDialog;
                    }
                    ChangeMsg::AddNewNodeToArray => {
                        let node_array = node.node.unwrap_array_mut();

                        let mut new_node = node_array.template(None);

                        if let Some(default) = &new_node.default {
                            new_node.apply_value(&default.clone(), false).unwrap();
                        }
                        new_node.modified = true;

                        match &mut node_array.values {
                            Some(values) => {
                                for n in &mut *values {
                                    n.modified = true;
                                }
                                values.push(new_node);
                            }
                            None => {
                                node_array.values = Some(vec![new_node]);
                            }
                        }
                        self.tree.set_modified(data_path.iter());
                    }

                    ChangeMsg::RenameKey { prev, new } => {
                        let node_object = node.node.unwrap_object_mut();

                        if node_object.nodes.contains_key(&new) {
                            return Action::None;
                        }

                        let node = node_object.nodes.get(&prev).unwrap().clone();
                        node_object.nodes.insert(new, node);
                        node_object.nodes.swap_remove(&prev);
                        self.tree.set_modified(data_path.iter());
                        action = Action::RemoveDialog;
                    }
                }

                self.data_path.sanitize_path(&self.tree);

                if self.tree.is_valid() {
                    self.write().unwrap();
                }
            }
             */
            PageMsg::None => {
                // pass
            }
            PageMsg::DialogAddNewNodeToObject(data_path) => {
                return Action::CreateDialog(Dialog::AddNewNodeToObject {
                    name: String::new(),
                    data_path,
                    page_id,
                });
            }
            PageMsg::DialogRenameKey(data_path, key) => {
                return Action::CreateDialog(Dialog::RenameKey {
                    previous: key.clone(),
                    name: key,
                    data_path,
                    page_id,
                });
            }
        };

        action
    }
}

pub fn create_pages(config: &Config) -> impl Iterator<Item = Page> + use<'_> {
    #[allow(clippy::vec_init_then_push)]
    fn default_paths() -> impl Iterator<Item = PathBuf> {
        let mut data_dirs: Vec<PathBuf> = vec![];
        #[cfg(target_os = "linux")]
        let base_dirs = xdg::BaseDirectories::new();
        #[cfg(target_os = "linux")]
        data_dirs.push(base_dirs.get_data_home().unwrap());
        #[cfg(target_os = "linux")]
        data_dirs.append(&mut base_dirs.get_data_dirs());

        #[cfg(debug_assertions)]
        data_dirs.push(PathBuf::from("test_schemas"));

        data_dirs.into_iter().map(|d| d.join("configurator"))
    }

    fn cosmic_compat(config: &Config) -> Box<dyn Iterator<Item = Page> + '_> {
        if config.cosmic_compat {
            let dir = include_dir!("$CARGO_MANIFEST_DIR/../cosmic_compat/schemas");

            Box::new(dir.entries().iter().filter_map(|entry| {
                let file = entry.as_file().unwrap();

                let content = file.contents_utf8().unwrap();

                let appid = appid_from_schema_path(file.path())?;

                if !config.masked.contains(&appid) {
                    Some(Page::from_str(&appid, content).unwrap())
                } else {
                    None
                }
            }))
        } else {
            Box::new(iter::empty())
        }
    }

    fn schema_test_path() -> impl Iterator<Item = PathBuf> {
        #[cfg(debug_assertions)]
        {
            iter::once(PathBuf::from(format!(
                "{}/test_schemas",
                env!("CARGO_MANIFEST_DIR")
            )))
        }

        #[cfg(not(debug_assertions))]
        {
            iter::empty()
        }
    }

    default_paths()
        .chain(schema_test_path())
        .filter_map(|xdg_path| fs::read_dir(xdg_path).ok())
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let schema_path = entry.path();
            let appid = appid_from_schema_path(&schema_path)?;

            if !config.masked.contains(&appid) {
                match fs::read_to_string(&schema_path) {
                    Ok(content) => match Page::from_str(&appid, &content) {
                        Ok(page) => Some(page),
                        Err(e) => {
                            error!("{}", e);
                            None
                        }
                    },
                    Err(e) => {
                        error!("{}", e);
                        None
                    }
                }
            } else {
                None
            }
        })
        .chain(cosmic_compat(config))
}

fn appid_from_schema_path(schema_path: &Path) -> Option<String> {
    let schema_name = schema_path.file_name().unwrap().to_string_lossy();

    if schema_name.starts_with('.') {
        return None;
    }

    schema_name.strip_suffix(".json").map(ToString::to_string)
}
