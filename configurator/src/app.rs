use std::{
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::bail;
use cosmic::{
    app::{Core, Task},
    executor,
    iced_widget::text,
    widget::{self, button, segmented_button::SingleSelectModel},
    Element,
};
use figment::{
    providers::{self, Format},
    Figment, Provider,
};
use figment_schemars_bridge::JsonSchemaProvider;
use json::Value;
use schemars::schema::RootSchema;
use xdg::BaseDirectories;
use zconf2::ConfigManager;

use crate::{
    config::{Config, CONFIG_PATH, SCHEMAS_PATH},
    fl,
    message::{AppMsg, ChangeMsg, PageMsg},
    node::{data_path::DataPath, Node, NodeContainer, NumberKind, NumberValue},
    view::view_app,
};

pub const QUALIFIER: &str = "io.github";
pub const ORG: &str = "wiiznokes";
pub const APP: &str = "configurator";
pub const APPID: &str = "io.github.wiiznokes.configurator";

pub struct App {
    core: Core,
    pub nav_model: SingleSelectModel,
    pub config: ConfigManager<Config>,
}

#[derive(Debug)]
pub struct Page {
    pub title: String,

    pub source_paths: Vec<PathBuf>,
    pub source_home_path: PathBuf,
    pub write_path: PathBuf,
    pub format: String,

    pub schema: RootSchema,

    pub system_config: Figment,
    pub user_config: Figment,
    pub full_config: Figment,

    pub tree: NodeContainer,
    pub data_path: DataPath,
}

struct BoxedProvider(Box<dyn Provider>);

impl Provider for BoxedProvider {
    fn metadata(&self) -> figment::Metadata {
        self.0.metadata()
    }

    fn data(
        &self,
    ) -> Result<figment::value::Map<figment::Profile, figment::value::Dict>, figment::Error> {
        self.0.data()
    }
}

fn provider_from_format(path: &Path, format: &str) -> anyhow::Result<BoxedProvider> {
    let provider = match format {
        "json" => providers::Json::file(path),

        _ => bail!("unknown format: {}", format),
    };

    Ok(BoxedProvider(Box::new(provider)))
}

impl Page {
    fn new(path: &Path) -> anyhow::Result<Self> {
        let json_value = {
            let mut file = File::open(path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            json::value::Value::from_str(&contents).unwrap()
        };

        let Some(json_obj) = json_value.as_object() else {
            bail!("no obj")
        };

        let source_paths = {
            if let Some(Value::String(paths)) = json_obj.get("X_CONFIGURATOR_SOURCE_PATHS") {
                paths.split_terminator(';').map(PathBuf::from).collect()
            } else {
                vec![]
            }
        };

        let source_home_path = {
            if let Some(Value::String(path)) = json_obj.get("X_CONFIGURATOR_SOURCE_HOME_PATH") {
                PathBuf::from(path)
            } else {
                bail!("no X_CONFIGURATOR_SOURCE_HOME_PATH")
            }
        };

        let write_path = {
            if let Some(Value::String(path)) = json_obj.get("X_CONFIGURATOR_WRITE_PATH") {
                PathBuf::from(path)
            } else {
                source_home_path.clone()
            }
        };

        let format = {
            if let Some(Value::String(format)) = json_obj.get("X_CONFIGURATOR_FORMAT") {
                format.clone()
            } else {
                source_home_path
                    .extension()
                    .expect("no format defined")
                    .to_str()
                    .unwrap()
                    .to_string()
            }
        };

        let mut system_config = Figment::new();

        for path in &source_paths {
            system_config = system_config.merge(provider_from_format(path, &format)?)
        }

        let user_config = Figment::new().merge(provider_from_format(&source_home_path, &format)?);

        let full_config = Figment::new()
            .merge(system_config.clone())
            .merge(user_config.clone());

        let json_schema: RootSchema = json::from_value(json_value)?;

        let mut tree = NodeContainer::from_json_schema(&json_schema);

        dbg!(&tree);

        tree.apply_figment(&full_config).unwrap();

        assert!(tree.is_valid());

        // dbg!(&tree);

        let title = path.file_name().unwrap().to_string_lossy().to_string();
        Ok(Self {
            title,
            schema: json_schema,
            user_config,
            system_config,
            full_config,
            tree,
            data_path: DataPath::new(),
            source_paths,
            source_home_path,
            write_path,
            format,
        })
    }

    fn title(&self) -> String {
        self.title.clone()
    }
}

impl cosmic::Application for App {
    type Executor = executor::Default;
    type Message = AppMsg;
    type Flags = ();

    const APP_ID: &'static str = APPID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let mut nav_model = SingleSelectModel::default();

        pub fn default_paths() -> impl Iterator<Item = PathBuf> {
            let base_dirs = BaseDirectories::new().unwrap();
            let mut data_dirs: Vec<PathBuf> = vec![];
            data_dirs.push(base_dirs.get_data_home());
            data_dirs.append(&mut base_dirs.get_data_dirs());

            data_dirs.into_iter().map(|d| d.join("configurator"))
        }

        for xdg_path in default_paths() {
            if let Ok(dir) = fs::read_dir(xdg_path) {
                for entry in dir {
                    let entry = entry.unwrap();

                    let page = Page::new(&entry.path()).unwrap();

                    // dbg!(&page);

                    nav_model.insert().text(page.title()).data::<Page>(page);
                }
            }
        }

        nav_model.activate_position(0);

        // let config = Config::default();

        let config = ConfigManager::new(QUALIFIER, ORG, APP).unwrap();

        dbg!(&config);

        let app = App {
            core,
            nav_model,
            config,
        };

        // let cmd = cosmic::app::command::message::cosmic(cosmic::app::cosmic::Message::Close);
        let cmd = Task::none();
        (app, cmd)
    }

    fn nav_model(&self) -> Option<&widget::nav_bar::Model> {
        Some(&self.nav_model)
    }

    fn on_nav_select(&mut self, id: widget::nav_bar::Id) -> Task<Self::Message> {
        self.nav_model.activate(id);
        Task::none()
    }

    fn view(&self) -> Element<Self::Message> {
        view_app(self)
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            AppMsg::ConfigActive(value) => {
                self.config.update(|settings| {
                    settings.active = value;
                });
            }
            AppMsg::ReloadLocalConfig => {
                self.config.reload().unwrap();
            }
            AppMsg::PageMsg(id, page_msg) => {
                if let Some(page) = self.nav_model.data_mut::<Page>(id) {
                    match page_msg {
                        PageMsg::SelectDataPath(pos) => {
                            page.data_path.change_to(pos);
                        }
                        PageMsg::OpenDataPath(data_path_type) => {
                            page.data_path.open(data_path_type);
                        }
                        PageMsg::ChangeMsg(data_path, change_msg) => {
                            let node = page.tree.get_at_mut(data_path.iter()).unwrap();

                            match change_msg {
                                ChangeMsg::ApplyDefault => {
                                    node.remove_value_rec();
                                    node.apply_value(node.default.clone().unwrap(), false)
                                        .unwrap();
                                }
                                ChangeMsg::ChangeBool(value) => {
                                    let node_bool = node.node.unwrap_bool_mut();
                                    node_bool.value = Some(value);
                                }
                                ChangeMsg::ChangeString(value) => {
                                    let node_string = node.node.unwrap_string_mut();
                                    node_string.value = Some(value);
                                }
                                ChangeMsg::ChangeNumber(value) => {
                                    let node_number = node.node.unwrap_number_mut();

                                    match node_number.kind {
                                        NumberKind::Integer => {
                                            if let Ok(value) = value.parse() {
                                                node_number.value = Some(NumberValue::I128(value));
                                            }
                                        }
                                        NumberKind::Float => {
                                            if let Ok(value) = value.parse() {
                                                node_number.value = Some(NumberValue::F64(value));
                                            }
                                        }
                                    }
                                    node_number.value_string = value;
                                }
                                ChangeMsg::ChangeEnum(value) => {
                                    let node_enum = node.node.unwrap_enum_mut();
                                    node_enum.value = Some(value);
                                }
                            }
                        }
                        PageMsg::None => {
                            // pass
                        }
                    }
                }
            }
        };

        let a = self.nav_model.active_data::<Page>().unwrap();

        // dbg!(&a.data_path);

        Task::none()
    }

    fn header_center(&self) -> Vec<Element<Self::Message>> {
        vec![
            // text(fl!("hello")).into(),
            // button::text(format!("is active = {}", self.config.active))
            //     .on_press(AppMsg::ConfigActive(!self.config.active))
            //     .into(),
            // button::text("reload local config")
            //     .on_press(AppMsg::ReloadLocalConfig)
            //     .into(),
        ]
    }
}
