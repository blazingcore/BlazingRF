use crate::utils::server::start_server;
use actix_web::rt::Runtime;

mod apps;
mod config;
mod macros;
mod utils;

pub struct Blast {
    secret: String,
    pub config: config::server::Config,
}

impl Default for Blast {
    fn default() -> Self {
        let secret = utils::generate_secret();
        Self {
            secret: secret.clone(),
            config: config::server::Config::init(secret),
        }
    }
}

impl Blast {
    pub fn new(port: Option<u16>) -> Self {
        let mut instance = Blast::default();
        let mut sys = config::System::init();
        if let Some(port) = port {
            sys.server.set_port(port);
            sys.save_config().unwrap();
            instance.config = sys.server;
        };
        instance
    }

    pub fn get_secret(&self) -> &str {
        &self.secret
    }

    pub fn set_port(&mut self, port: u16) {
        self.config.set_port(port);
    }

    pub fn get_config(&self) -> &config::server::Config {
        &self.config
    }

    pub fn get_debug(&self) -> bool {
        self.config.get_debug()
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.config.set_debug(debug);
    }

    pub fn run(&self) {
        let port = self.get_config().get_port();
        if self.get_debug() {
            println!("Running in debug mode...");
            println!("secret: {}", self.get_secret());
            println!("port: {}", port);
        } else {
            println!("Running...");
        }
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            start_server(port).await.unwrap();
        });
    }
}
