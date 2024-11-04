use std::{
    fmt::Debug,
    fs::{create_dir_all, File},
    io::{Read, Write},
    mem,
    path::{Path, PathBuf},
    sync::{Arc, OnceLock, RwLock},
};

use backend::{backend, BackEnd};
use figment::{providers, Figment};
use figment_schemars_bridge::FigmentSerdeBridge;
use serde::{Deserialize, Serialize};

mod backend;

#[derive(Default, Clone)]
struct ConfigHandler {
    figment: Figment,
    appid: String,
    path: PathBuf,
}

impl ConfigHandler {
    fn config_path(&self) -> PathBuf {
        let backend = backend();

        self.path
            .join(format!("{}{}", self.appid, backend.extension()))
    }
}

static CONFIG_HANDLER: OnceLock<Arc<RwLock<ConfigHandler>>> = OnceLock::new();

impl Debug for ConfigHandler {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

fn read_from_path(path: &Path) -> Figment {
    let backend = backend();

    let mut string = String::new();

    if let Ok(mut file) = File::open(path) {
        file.read_to_string(&mut string).unwrap();

        Figment::new().merge(backend.figment_from(&string).unwrap())
    } else {
        Figment::new()
    }
}

pub fn init<'a, Config: Deserialize<'a>>(appid: &'static str, path: PathBuf) -> Config {
    let figment = Figment::new();

    let mut config_handler = ConfigHandler {
        figment,
        appid: appid.to_string(),
        path,
    };

    config_handler.figment = read_from_path(&config_handler.config_path());

    let res = config_handler.figment.extract().unwrap();

    CONFIG_HANDLER
        .set(Arc::new(RwLock::new(config_handler)))
        .unwrap();

    res
}

pub fn write<V>(name: &str, value: &V)
where
    V: Serialize,
{
    let backend = backend();

    match CONFIG_HANDLER.get().cloned() {
        Some(config_handler) => {
            let mut config_handler = config_handler.write().unwrap();
            create_dir_all(&config_handler.path).unwrap();

            let figment = mem::replace(&mut config_handler.figment, Figment::new());

            config_handler.figment = figment.merge(providers::Serialized::default(name, value));

            let serde_bridge = FigmentSerdeBridge::new(&config_handler.figment);

            let e = backend.serialise(&serde_bridge).unwrap();

            dbg!(&config_handler.figment);

            let mut file = File::create(config_handler.path.join(format!(
                "{}{}",
                config_handler.appid,
                backend.extension()
            )))
            .unwrap();

            file.write_all(e.as_bytes()).unwrap();
        }
        None => todo!(),
    }
}

pub fn load<'a, Config: Deserialize<'a>>() -> Config {
    match CONFIG_HANDLER.get().cloned() {
        Some(config_handler) => {
            let mut config_handler = config_handler.write().unwrap();

            config_handler.figment = read_from_path(&config_handler.config_path());

            config_handler.figment.extract().unwrap()
        }
        None => Figment::new().extract().unwrap(),
    }
}
