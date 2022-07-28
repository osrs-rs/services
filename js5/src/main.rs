use config::Config;
use rscache::Cache;
use std::sync::{Arc, Mutex};

mod config;
mod js5;
mod network;

#[tokio::main]
async fn main() {
    // Load config
    let config_file_string = std::fs::read_to_string("js5.toml").expect("failed opening js5.toml");
    let config: &mut Arc<Mutex<Config>> =
        &mut Arc::new(Mutex::new(toml::from_str(&config_file_string).unwrap()));

    // Load cache
    let cache = &mut Arc::new(Cache::new("cache").expect("could not load cache"));

    // Create socket acceptor for accepting connections, loop forever
    network::accept_js5_sockets(cache, config).await.unwrap();
}
