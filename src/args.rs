use clap::{Args, Subcommand, Parser};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RuzkyArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create a static rest api to test frontends
    #[clap(name = "server")]
    Server(ServerArgs),
}

#[derive(Debug, Args)]
pub struct ServerArgs {
    /// Init ruzky server file
    #[clap(short, long, conflicts_with = "start")]
    pub init: bool,
    /// Select an api template (Blog/Todo/Profiles)
    #[clap(short, long, conflicts_with = "start")]
    pub template: Option<String>,
    /// Start ruzky server
    #[clap(short, long, conflicts_with = "init")]
    pub start: bool,
}
