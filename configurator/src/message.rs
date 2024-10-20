use cosmic::widget::segmented_button::Entity;

use crate::node::data_path::DataPathType;

#[derive(Clone, Debug)]
pub enum AppMsg {
    PageMsg(Entity, PageMsg),
    ReloadActivePage,
    ReloadLocalConfig,
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
