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
                    page.update(page_msg);
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
