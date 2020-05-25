use std::io;
use std::io::BufRead;
use std::io::Write;

extern crate config;
extern crate chrono;
extern crate serde;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

mod settings;
mod db;
pub mod schema;
pub mod models;

use settings::Settings;

fn main() {
    let config = Settings::new();
    let config = match config {
        Ok(settings) => settings,
        Err(err) => panic!("Error parsing settings: {}", err),
    };

    // TODO setup db
    let db_conn = db::establish_connection(config.database);

    start_cli();
    println!("");
    println!("Exiting...");
}

fn start_cli() {
    println!("Starting Freetote CLI");
    print!("freetote> ");
    flush_streams();
    for line in io::stdin().lock().lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!("Failed to read line {}", err),
        };
        if line == "\n" {
            continue;
        }
        let command: Vec<&str> = line.trim().split_whitespace().collect();
        match command[0] {
            "exit" | "bye" | "quit" => break,
            _ => (),
        }
        println!("Command: {:?}", command);
        print!("freetote> ");
        flush_streams();
    }
}

fn flush_streams() {
    io::stdout().flush().ok().expect("Failed to flush stdout");
    io::stderr().flush().ok().expect("Failed to flush stderr");
}
