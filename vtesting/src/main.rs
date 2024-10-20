use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct Config {
    pub private_mode: bool,
    pub maximum_entries_lifetime: Option<u64>,
    pub maximum_entries_number: Option<u32>,
    pub horizontal: bool,
    pub unique_session: bool,
}

fn main() {}
