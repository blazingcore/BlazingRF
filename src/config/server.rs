use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    port: u16,
    secret: String,
    debug: bool,
}

impl Config {
    pub fn init(secret: String) -> Self {
        Self {
            port: 8080,
            secret,
            debug: false,
        }
    }

    pub fn get_secret(&self) -> &str {
        &self.secret
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub fn get_debug(&self) -> bool {
        self.debug
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }
}
