use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(Deserialize, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub enable_demo_mode: bool,
}

#[derive(Deserialize, Clone)]
pub struct DatabaseSettings {
    pub filename: String,
    pub enable_in_memory: bool,
}
