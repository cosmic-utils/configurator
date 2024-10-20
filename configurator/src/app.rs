use cosmic::{
    app::{Core, Task},
    executor,
    iced_widget::text_input,
    widget::{
        self, button,
        segmented_button::{Entity, SingleSelectModel},
        text,
    },
    Element,
};
use zconf2::ConfigManager;

use crate::{
    config::Config,
    message::{AppMsg, ChangeMsg, PageMsg},
    node::{data_path::DataPathType, NumberKind, NumberValue},
    page::{self, create_pages, Page},
    view::view_app,
};

pub const QUALIFIER: &str = "io.github";
pub const ORG: &str = "wiiznokes";
pub const APP: &str = "configurator";
pub const APPID: &str = "io.github.wiiznokes.configurator";

#[derive(Debug)]
pub enum Dialog {
    AddNewNodeToObject {
        name: String,
        data_path: Vec<DataPathType>,
        page_id: Entity,
    },
    RenameKey {
        previous: String,
        name: String,
        data_path: Vec<DataPathType>,
        page_id: Entity,
    },
}

pub struct App {
    core: Core,
    pub nav_model: SingleSelectModel,
    pub config: ConfigManager<Config>,
    pub dialog: Option<Dialog>,
}

impl App {
    fn close_dialog(&mut self) {
        self.dialog.take();
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
        let config: ConfigManager<Config> = ConfigManager::new(QUALIFIER, ORG, APP).unwrap();

        let mut nav_model = SingleSelectModel::default();

        let mut active = false;

        for page in create_pages() {
            if let Some(appid) = &config.settings().last_used_page
                && appid == &page.appid
            {
                let entity = nav_model
                    .insert()
                    .text(page.title())
                    .data::<Page>(page)
                    .id();
                nav_model.activate(entity);
                active = true;
            } else {
                nav_model.insert().text(page.title()).data::<Page>(page);
            }
        }

        if !active {
            nav_model.activate_position(0);
        }

        let app = App {
            core,
            nav_model,
            config,
            dialog: None,
        };

        (app, Task::none())
    }

    fn nav_model(&self) -> Option<&widget::nav_bar::Model> {
        Some(&self.nav_model)
    }

    fn on_nav_select(&mut self, id: widget::nav_bar::Id) -> Task<Self::Message> {
        self.nav_model.activate(id);

        let page: &Page = self.nav_model.data(self.nav_model.active()).unwrap();

        self.config.update(|s| {
            s.last_used_page = Some(page.appid.clone());
        });
        Task::none()
    }

    fn view(&self) -> Element<Self::Message> {
        view_app(self)
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            AppMsg::PageMsg(id, page_msg) => {
                if let Some(page) = self.nav_model.data_mut::<Page>(id) {
                    match page.update(page_msg, id) {
                        page::Action::CreateDialog(dialog) => {
                            self.dialog.replace(dialog);
                        }
                        page::Action::None => {}
                        page::Action::RemoveDialog => {
                            self.close_dialog();
                        }
                    };
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
            AppMsg::CloseDialog => {
                self.close_dialog();
            }
            AppMsg::DialogInput(input) => match self.dialog.as_mut().unwrap() {
                Dialog::AddNewNodeToObject {
                    name,
                    data_path,
                    page_id,
                } => {
                    *name = input;
                }
                Dialog::RenameKey {
                    previous,
                    name,
                    data_path,
                    page_id,
                } => {
                    *name = input;
                }
            },
        };

        // let a = self.nav_model.active_data::<Page>().unwrap();
        // dbg!(&a.data_path);

        Task::none()
    }

    fn dialog(&self) -> Option<Element<Self::Message>> {
        self.dialog.as_ref().map(|dialog| match dialog {
            Dialog::AddNewNodeToObject {
                name,
                data_path,
                page_id,
            } => widget::dialog("Create")
                .control(text_input("name", name).on_input(AppMsg::DialogInput))
                .primary_action(button::text("create").on_press(AppMsg::PageMsg(
                    *page_id,
                    PageMsg::ChangeMsg(
                        data_path.clone(),
                        ChangeMsg::AddNewNodeToObject(name.clone()),
                    ),
                )))
                .secondary_action(button::text("cancel").on_press(AppMsg::CloseDialog))
                .into(),
            Dialog::RenameKey {
                previous,
                name,
                data_path,
                page_id,
            } => widget::dialog("Rename")
                .control(text_input("name", name).on_input(AppMsg::DialogInput))
                .primary_action(button::text("rename").on_press(AppMsg::PageMsg(
                    *page_id,
                    PageMsg::ChangeMsg(
                        data_path.clone(),
                        ChangeMsg::RenameKey {
                            prev: previous.clone(),
                            new: name.clone(),
                        },
                    ),
                )))
                .secondary_action(button::text("cancel").on_press(AppMsg::CloseDialog))
                .into(),
        })
    }

    fn header_end(&self) -> Vec<Element<Self::Message>> {
        vec![button::text("reload")
            .on_press(AppMsg::ReloadActivePage)
            .into()]
    }
}
