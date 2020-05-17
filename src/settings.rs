use config::{Config, ConfigError, Environment, File};
use std::env;

const CONF_PATH: &str = "./config/";

#[derive(Debug, Deserialize)]
pub struct Database {
    url: String,
    database: String,
    user: String,
    passwd: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    database: Database,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut cfg = Config::new();
        cfg.merge(File::with_name(&format!("{}{}", CONF_PATH, "default.yml")));

        // TODO add user defined configs

        cfg.try_into()
    }
}
