use clap::{Parser, command, Subcommand};

#[derive(Parser)]
#[command(name = "raincloud")]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Remote,
    Local,
}

#[derive(Debug)]
struct Remote {
    url: String,
}

#[derive(Debug)]
struct Local {
    path: String
}


pub async fn entrypoint() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Remote => todo!(),
        Commands::Local => todo!(),
    }
}
