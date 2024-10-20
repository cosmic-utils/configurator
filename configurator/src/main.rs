// #![feature(btree_extract_if)]
// #![feature(if_let_guard)]
#![feature(let_chains)]

use app::App;
use cosmic::app::Settings;

#[allow(unused_imports)]
#[macro_use]
extern crate tracing;

mod app;
mod config;
mod figment_serde_bridge;
mod localize;
mod message;
mod node;
mod page;
mod utils;
mod view;
#[macro_use]
mod icon;
#[cfg(test)]
mod json_schema_test_suite;
#[cfg(test)]
mod test_schema;

fn setup_logs() {
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new(format!(
        "warn,{}=info",
        env!("CARGO_CRATE_NAME")
    )));

    if let Ok(journal_layer) = tracing_journald::layer() {
        tracing_subscriber::registry()
            .with(filter_layer)
            .with(fmt_layer)
            .with(journal_layer)
            .init();
    } else {
        tracing_subscriber::registry()
            .with(filter_layer)
            .with(fmt_layer)
            .init();
    }
}

fn main() -> cosmic::iced::Result {
    localize::localize();
    setup_logs();

    cosmic::app::run::<App>(Settings::default(), ())
}
