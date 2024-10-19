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

#[derive(Debug)]
pub struct Page {
    pub title: String,

    pub source_paths: Vec<PathBuf>,
    pub source_home_path: PathBuf,
    pub write_path: PathBuf,
    pub format: String,

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

        data_dirs.into_iter().map(|d| d.join("configurator"))
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

        let tree = NodeContainer::from_json_schema(&json::from_value(json_value)?);

        let title = path.file_name().unwrap().to_string_lossy().to_string();

        let mut page = Self {
            title,
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

        page.reload().unwrap();

        Ok(page)
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn reload(&mut self) -> anyhow::Result<()> {
        self.user_config =
            Figment::new().merge(provider_from_format(&self.source_home_path, &self.format)?);

        self.full_config = Figment::new()
            .merge(self.system_config.clone())
            .merge(self.user_config.clone());

        self.tree.apply_figment(&self.full_config)?;

        assert!(self.tree.is_valid());

        Ok(())
    }
}
