use cosmic::widget::segmented_button::Entity;

use crate::node::data_path::{DataPath, DataPathType};

#[derive(Clone, Debug)]
pub enum AppMsg {
    ConfigActive(bool),
    ReloadLocalConfig,
    PageMsg(Entity, PageMsg),
}

#[derive(Clone, Debug)]
pub enum PageMsg {
    SelectDataPath(Option<usize>),
    OpenDataPath(DataPathType),
    ChangeMsg(Vec<DataPathType>, ChangeMsg),
    None,
}

#[derive(Clone, Debug)]
pub enum ChangeMsg {
    ApplyDefault,
    ChangeBool(bool),
    ChangeString(String),
    ChangeNumber(String),
    ChangeEnum(usize),
}
