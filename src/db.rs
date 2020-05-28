use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::settings;
use crate::models::*;

pub struct FTDB {
    pub conn: PgConnection,
}

pub fn new_ftdb(db_settings: settings::Database) -> FTDB {
    let pw_str = if db_settings.passwd == "" {
        String::from("")
    } else {
        format!(":{}", db_settings.passwd)
    };

    let db_url = format!(
        "postgres://{}{}@{}/{}",
        db_settings.user,
        pw_str,
        db_settings.url,
        db_settings.dbname
    );
    FTDB {
        conn: PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url)),
    }
}

impl FTDB {
}

