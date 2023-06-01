use clap::{Args, Subcommand, Parser};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct RuzkyArgs {
    #[clap(subcommand)]
    entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
enum EntityType {
    /// Create a static rest api to test frontends
    #[clap(name = "server")]
    Server(ServerArgs),
}

#[derive(Debug, Args)]
struct ServerArgs {
    /// Init ruzky server file
    #[clap(short, long, conflicts_with = "start")]
    init: bool,
    /// Start ruzky server
    #[clap(short, long, conflicts_with = "init")]
    start: bool,
}

fn main() {
    let args = RuzkyArgs::parse();

    match args.entity_type {
        EntityType::Server(server_args) => {
            if server_args.init {
                init_server();
            } else if server_args.start {
                start_server();
            }
        }
    }
}

fn init_server() {
    println!("Initializing server...");
}

fn start_server() {
    println!("Starting server...");
}
