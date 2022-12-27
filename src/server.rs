use std::{net::SocketAddr, str::FromStr};

use crate::{config::Config, git::clone};

pub async fn handle_remote(mut cfg: Config) {
    let clone_dir = match clone(&cfg) {
        Ok(val) => val,
        Err(err) => panic!("{}", err),
    };
    cfg.directory += &clone_dir;
    return start(cfg).await;
}
pub async fn handle_local(cfg: Config) {
    return start(cfg).await;
}

async fn start(cfg: Config) {
    let bind_address =
        match SocketAddr::from_str(&format!("{}:{}", cfg.server.bind_address, cfg.server.port)) {
            Ok(val) => val,
            Err(_) => panic!("Could not parse bind_address!"),
        };
    println!("Serving {} on {}", &cfg.directory, bind_address);
    warp::serve(warp::fs::dir(cfg.directory))
        .run(bind_address)
        .await;
}
