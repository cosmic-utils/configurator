use std::{
    fs::{self, File},
    io::Read,
    iter,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{anyhow, bail};
use cosmic::widget::segmented_button::Entity;
use directories::BaseDirs;
use figment::{
    providers::{self, Format},
    value::{Dict, Tag, Value},
    Figment, Profile, Provider,
};

use xdg::BaseDirectories;

use crate::{
    app::Dialog,
    message::{ChangeMsg, PageMsg},
    node::{data_path::DataPath, Node, NodeContainer, NumberValue},
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

    pub system_config: Figment,
    pub user_config: Figment,
    pub full_config: Figment,

    pub tree: NodeContainer,
    pub data_path: DataPath,
}

pub fn create_pages() -> impl Iterator<Item = Page> {
    fn default_paths() -> impl Iterator<Item = PathBuf> {
        let base_dirs = BaseDirectories::new().unwrap();
        let mut data_dirs: Vec<PathBuf> = vec![];
        data_dirs.push(base_dirs.get_data_home());
        data_dirs.append(&mut base_dirs.get_data_dirs());

        #[cfg(debug_assertions)]
        data_dirs.push(PathBuf::from("test_schemas"));

        let r = data_dirs.into_iter().map(|d| d.join("configurator"));

        #[cfg(debug_assertions)]
        {
            r.chain(iter::once(PathBuf::from("configurator/test_schemas")))
        }

        #[cfg(not(debug_assertions))]
        {
            r
        }
    }

    default_paths()
        .filter_map(|xdg_path| fs::read_dir(xdg_path).ok())
        .flatten()
        .flatten()
        .map(|entry| Page::new(&entry.path()).unwrap())
}

impl Page {
    fn new(path: &Path) -> anyhow::Result<Self> {
        let json_value = {
            let mut file = File::open(path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            json::Value::from_str(&contents).unwrap()
        };

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

        let mut system_config = Figment::new();

        for path in &source_paths {
            system_config = system_config.merge(crate::providers::from_format(path, &format))
        }

        let tree = NodeContainer::from_json_schema(&json::from_value(json_value)?);

        let schema_name = path.file_name().unwrap().to_string_lossy();

        let appid = schema_name.strip_suffix(".json").unwrap().to_string();

        let title = appid.split('.').last().unwrap().to_string();

        let mut page = Self {
            title,
            appid,
            system_config,
            user_config: Figment::new(),
            full_config: Figment::new(),
            tree,
            data_path: DataPath::new(),
            source_paths,
            source_home_path,
            write_path,
            format,
        };

        // dbg!(&page.tree);

        if let Err(err) = page.reload() {
            error!("{err}");
        }

        Ok(page)
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn reload(&mut self) -> anyhow::Result<()> {
        self.user_config = Figment::new().merge(crate::providers::from_format(
            &self.source_home_path,
            &self.format,
        ));

        self.full_config = Figment::new()
            .merge(self.system_config.clone())
            .merge(self.user_config.clone());

        // dbg!(&self.tree);
        // dbg!(&self.full_config);

        self.tree.apply_figment(&self.full_config)?;

        self.data_path.sanitize_path(&self.tree);

        Ok(())
    }

    pub fn write(&self) -> anyhow::Result<()> {
        match self.tree.to_value(&Tag::Default) {
            Some(value) => {
                crate::providers::write(&self.write_path, &self.format, &value)?;
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
        let mut action = Action::None;

        match message {
            PageMsg::SelectDataPath(pos) => {
                self.data_path.change_to(pos);
            }
            PageMsg::OpenDataPath(data_path_type) => {
                self.data_path.open(data_path_type);
            }
            PageMsg::ChangeMsg(data_path, change_msg) => {
                let node = self.tree.get_at_mut(data_path.iter()).unwrap();

                match change_msg {
                    ChangeMsg::ApplyDefault => {
                        node.remove_value_rec();
                        node.apply_value(node.default.clone().unwrap(), false)
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

                        if let Some(default) = &new_node.default {
                            new_node.apply_value(default.clone(), false).unwrap();
                        } else {
                            new_node
                                .apply_value(Value::Dict(Tag::Default, Dict::new()), false)
                                .unwrap();
                        }

                        node_object.nodes.insert(name, new_node);

                        for n in node_object.nodes.values_mut() {
                            n.modified = true;
                        }

                        self.tree.set_modified(data_path.iter());

                        action = Action::RemoveDialog;
                    }
                    ChangeMsg::AddNewNodeToArray => {
                        let node_array = node.node.unwrap_array_mut();

                        let mut new_node = node_array.template();

                        if let Some(default) = &new_node.default {
                            new_node.apply_value(default.clone(), false).unwrap();
                        } else {
                            new_node
                                .apply_value(Value::Dict(Tag::Default, Dict::new()), false)
                                .unwrap();
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
