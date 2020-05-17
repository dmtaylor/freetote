#[macro_use]

use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::settings::Database;
//mod settings;

pub fn establish_connection(db_settings: Database) -> PgConnection {
    // TODO

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
    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))

}

