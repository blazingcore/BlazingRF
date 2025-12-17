mod cli;
mod macros;
mod utils;

use actix_web::rt::Runtime;
use blast::Blast;
use clap::Parser;
use cli::{Cli, Command};

use crate::utils::server::start_server;

fn main() {
    let mut server = Blast::new(None);
    let cli = Cli::parse();

    match cli.command {
        Command::Create(create_command) => {
            println!("create command: {:?}", create_command);
        }
        Command::Start(start_args) => {
            if start_args.test {
                println!("Testing in progress...");
                server.set_debug(start_args.debug);
                std::thread::sleep(std::time::Duration::from_secs(1));

                print!("Testing server...");
                Runtime::new().unwrap().spawn(start_server(start_args.port));
                std::thread::sleep(std::time::Duration::from_secs(2));
                println!("Ok");

                println!("Testing complete!");
                return;
            }
            if start_args.port != 8000 {
                server.set_port(start_args.port);
            }
            if start_args.debug {
                server.set_debug(true);
            }
            server.run();
        }
    }
}
