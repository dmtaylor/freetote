use config::{Config, ConfigError, Environment, File};
use std::env;

const CONF_PATH: &str = "./config/";

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub url: String,
    pub dbname: String,
    pub user: String,
    pub passwd: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub database: Database,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut cfg = Config::new();
        cfg.set_default("database.passwd", "").unwrap();
        cfg.merge(File::with_name(&format!("{}{}", CONF_PATH, "default.yml"))).unwrap();

        // TODO add user defined configs

        cfg.try_into()
    }
}
