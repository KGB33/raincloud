use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use crate::config::Config;

pub async fn handle_remote(cfg: Config) {}
pub async fn handle_local(cfg: Config) {
    return start(cfg).await;
}

async fn start(cfg: Config) {
    let bind_address =
        match SocketAddr::from_str(&format!("{}:{}", cfg.server.bind_address, cfg.server.port)) {
            Ok(val) => val,
            Err(_) => panic!("Could not parse bind_address!"),
        };
    println!("Serving {} on {}", cfg.directory, bind_address);
    warp::serve(warp::fs::dir(cfg.directory))
        .run(bind_address)
        .await;
}
