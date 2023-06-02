pub mod server;
pub mod args;

use clap::Parser;
use server::{init::init_server, start::start_server};
use args::{EntityType, RuzkyArgs};

fn main() {
    let args = RuzkyArgs::parse();

    match args.entity_type {
        EntityType::Server(server_args) => {
            if server_args.init {
                match server_args.template {
                    Some(template) => init_server(template),
                    None => init_server("default".to_string())
                }
            } else if server_args.start {
                start_server();
            } else if server_args.template.is_some() {
                println!("no puedes utilizar este argumento solo")
            } else {
                start_server();
            }
        }
    }
}
