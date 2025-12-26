use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
}

impl Config {
    pub fn init() -> Self {
        Self {
            host: String::from("localhost"),
            port: 5432,
            username: String::from("postgres"),
            password: String::from("postgres"),
            database: String::from("blast"),
        }
    }
}
