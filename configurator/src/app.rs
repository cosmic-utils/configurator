use cosmic::{
    app::{Core, Task},
    executor,
    widget::{self, button, segmented_button::SingleSelectModel},
    Element,
};
use zconf2::ConfigManager;

use crate::{
    config::Config,
    message::{AppMsg, ChangeMsg, PageMsg},
    node::{NumberKind, NumberValue},
    page::{create_pages, Page},
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

        for page in create_pages() {
            nav_model.insert().text(page.title()).data::<Page>(page);
        }

        nav_model.activate_position(0);

        let config = ConfigManager::new(QUALIFIER, ORG, APP).unwrap();

        let app = App {
            core,
            nav_model,
            config,
        };

        (app, Task::none())
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
                                    page.tree.set_modified(data_path.iter());
                                }
                                ChangeMsg::ChangeString(value) => {
                                    let node_string = node.node.unwrap_string_mut();
                                    node_string.value = Some(value);
                                    page.tree.set_modified(data_path.iter());
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
                                    page.tree.set_modified(data_path.iter());
                                }
                                ChangeMsg::ChangeEnum(value) => {
                                    let node_enum = node.node.unwrap_enum_mut();
                                    node_enum.value = Some(value);
                                    page.tree.set_modified(data_path.iter());
                                }
                            }

                            page.write().unwrap();
                        }
                        PageMsg::None => {
                            // pass
                        }
                    }
                }
            }
            AppMsg::ReloadActivePage => {
                if let Some(page) = self.nav_model.active_data_mut::<Page>() {
                    page.reload().unwrap();
                }
            }
            AppMsg::ReloadLocalConfig => {
                self.config.reload().unwrap();
            }
        };

        // let a = self.nav_model.active_data::<Page>().unwrap();
        // dbg!(&a.data_path);

        Task::none()
    }

    fn header_end(&self) -> Vec<Element<Self::Message>> {
        vec![button::text("reload")
            .on_press(AppMsg::ReloadActivePage)
            .into()]
    }
}
