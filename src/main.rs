mod config;
mod git;
mod server;
mod cli;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    cli::entrypoint().await;
}
