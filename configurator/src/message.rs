use cosmic::widget::segmented_button::Entity;

use crate::node::data_path::DataPathType;

#[derive(Clone, Debug)]
pub enum AppMsg {
    PageMsg(Entity, PageMsg),
    ReloadActivePage,
    ReloadLocalConfig,
    CloseDialog,
    DialogInput(String),
}

#[derive(Clone, Debug)]
pub enum PageMsg {
    SelectDataPath(Option<usize>),
    OpenDataPath(DataPathType),
    ChangeMsg(Vec<DataPathType>, ChangeMsg),
    DialogAddNewNodeToObject(Vec<DataPathType>),
    DialogRenameKey(Vec<DataPathType>, String),
    None,
}

#[derive(Clone, Debug)]
pub enum ChangeMsg {
    ApplyDefault,
    ChangeBool(bool),
    ChangeString(String),
    ChangeNumber(String),
    ChangeEnum(usize),
    Remove(DataPathType),
    AddNewNodeToObject(String),
    AddNewNodeToArray,
    RenameKey { prev: String, new: String },
}
