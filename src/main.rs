mod config;
mod git;
mod server;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    server::start().await;
}
