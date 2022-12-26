use clap::{command, Parser, Subcommand};

use crate::{
    config::Config,
    server::{handle_local, handle_remote},
};

#[derive(Parser)]
#[command(name = "raincloud")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Remote { url: String },
    Local { path: String },
}
pub async fn entrypoint() {
    let cli = Cli::parse();
    let cfg = Config::default();

    match &cli.command {
        Commands::Remote { url } => handle_remote(cfg).await,
        Commands::Local { path } => handle_local(cfg).await,
    }
}
