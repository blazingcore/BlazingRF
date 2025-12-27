use std::{
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

use crate::utils::generate_secret;

pub mod database;
pub mod server;

#[derive(Serialize, Deserialize, Debug)]
pub struct System {
    pub server: server::Config,
    pub database: database::Config,
    pub applications: Vec<String>,
}

impl System {
    pub fn init() -> Self {
        match File::open("config/main.toml") {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                toml::from_str(&contents).unwrap()
            }
            Err(_) => {
                let sys = Self {
                    server: server::Config::init(generate_secret()),
                    database: database::Config::init(),
                    applications: Vec::new(),
                };
                sys.save_config().unwrap();
                sys
            }
        }
    }

    pub fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.server.get_debug() {
            println!("Creating config directory...");
        }
        std::fs::create_dir_all("config")?;
        if self.server.get_debug() {
            println!("Creating config file...");
        }
        let mut file = File::create("config/main.toml")?;
        if self.server.get_debug() {
            println!("Writing config file...");
        }
        file.write_all(toml::to_string(self)?.as_bytes())?;
        Ok(())
    }

    pub fn backup_config(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.server.get_debug() {
            println!("Creating config directory...");
        }
        std::fs::create_dir_all(path)?;
        if self.server.get_debug() {
            println!("Copying config file...");
        }
        let mut file = File::create(format!("{}/main.toml", path))?;
        file.write_all(toml::to_string(self)?.as_bytes())?;
        Ok(())
    }

    pub fn restore_from_backup(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.server.get_debug() {
            println!("Finding backup files...");
        }
        let mut backup_file = File::open(format!("{}/main.toml", path))?;
        let mut contents = String::new();
        backup_file.read_to_string(&mut contents)?;
        let backup: Self = toml::from_str(&contents)?;
        backup.save_config()?;
        Ok(())
    }
}
