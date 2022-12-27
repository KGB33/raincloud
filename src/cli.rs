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
    let mut cfg = Config::default();

    match &cli.command {
        Commands::Remote { url } => {
            cfg.repo.remote = url.to_string();
            handle_remote(cfg).await;
        }
        Commands::Local { path } => {
            cfg.directory = path.to_string();
            handle_local(cfg).await;
        }
    }
}
