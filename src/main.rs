pub mod server;
pub mod args;

use clap::Parser;
use colored::Colorize;
use server::{init::{init_server, init_server_with_dir}, start::start_server};
use args::{EntityType, RuzkyArgs};

/// The main entry point of the Ruzky application.
fn main() {
    let args = RuzkyArgs::parse();

    match args.entity_type {
        EntityType::Server(server_args) => {
            if server_args.init {
                if let Some(dir) = server_args.directory {
                    // Handle initialization with directory argument
                    init_server_with_dir(dir)
                } else if let Some(template) = server_args.template {
                    // Handle initialization with template argument
                    init_server(&template);
                } else {
                    // Handle default initialization
                    init_server("default");
                }
            } else if server_args.start {
                // Start the server
                start_server();
            } else if server_args.template.is_some() || server_args.directory.is_some() {
                println!("{}", "You cannot use this argument alone.".red().bold().underline());
            } else {
                // Start the server by default
                start_server();
            }
        }
    }
}
