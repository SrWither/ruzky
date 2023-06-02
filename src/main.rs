pub mod server;
pub mod args;

use clap::Parser;
use server::{init::init_server, start::start_server};
use args::{EntityType, RuzkyArgs};

/// The main entry point of the Ruzky application.
fn main() {
    let args = RuzkyArgs::parse();

    match args.entity_type {
        EntityType::Server(server_args) => {
            if server_args.init {
                match server_args.template {
                    Some(template) => init_server(&template),
                    None => init_server("default")
                }
            } else if server_args.start {
                start_server();
            } else if server_args.template.is_some() {
                println!("You cannot use this argument alone.");
            } else {
                start_server();
            }
        }
    }
}
