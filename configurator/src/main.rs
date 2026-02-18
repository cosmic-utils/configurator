// #![feature(btree_extract_if)]
#![feature(if_let_guard)]

use app::App;
use cosmic::app::Settings;

#[allow(unused_imports)]
#[macro_use]
extern crate tracing;

mod app;
mod config;
mod generic_value;
mod localize;
mod message;
mod node;
mod page;
mod providers;
mod utils;
mod view;

#[macro_use]
mod icon;
// #[cfg(test)]
// mod json_schema_test_suite;
#[cfg(test)]
mod manual_testing;

#[cfg(test)]
mod test_common;


fn setup_log_for_test() {
    use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new(format!(
        "warn,{}=debug",
        env!("CARGO_CRATE_NAME")
    )));

    tracing_subscriber::registry()
            .with(filter_layer)
            .with(fmt_layer)
            .init();
}


fn setup_logs() {
    use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

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
