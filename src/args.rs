use clap::{Args, Subcommand, Parser};

/// Represents the command-line arguments for the Ruzky application.
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RuzkyArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

/// Represents the entity type for the Ruzky application.
#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create a static REST API to test frontends
    #[clap(name = "server")]
    Server(ServerArgs),
}

/// Represents the command-line arguments for the Ruzky server.
#[derive(Debug, Args)]
pub struct ServerArgs {
    /// Initialize the Ruzky server file
    #[clap(short, long, conflicts_with = "start")]
    pub init: bool,
    /// Select an API template (Blog/Todo/Profiles)
    #[clap(short, long, conflicts_with = "start")]
    pub template: Option<String>,
    /// Initialize with a directory with existing jsons files
    #[clap(short, long, conflicts_with_all = &["start", "template"])]
    pub directory: Option<String>,
    /// Start the Ruzky server
    #[clap(short, long, conflicts_with = "init")]
    pub start: bool,
}
