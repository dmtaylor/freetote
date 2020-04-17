use config::{Config, ConfigError, Environment, File};
use std::env;

const CONF_PATH: &str = "./config/";

#[derive(Debug, Deserialize)]
struct Database {
    dbtype: String,
    url: String,
    database: String,
    user: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    database: Database,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut cfg = Config::new();
        // TODO
        //
        //let default_conf = format!("{}{}", CONF_PATH, "default.yml");
        cfg.merge(File::with_name(&format!("{}{}", CONF_PATH, "default.yml")));

        cfg.try_into()
    }
}
