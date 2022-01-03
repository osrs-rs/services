use osrscache::Cache;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

use crate::{config::Config, js5};

pub async fn accept_js5_sockets(
    cache: &mut Arc<Cache>,
    config: &mut Arc<Mutex<Config>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_lock = config.lock().unwrap();

    let mut ip = "127.0.0.1:".to_string();
    ip.push_str(&config_lock.port.unwrap_or(43596).to_string());

    let revision = config_lock.revision;

    let listener = TcpListener::bind(&ip).await?;

    println!("Listening on {}...", &ip);

    loop {
        let (mut socket, _) = listener.accept().await?;

        let mut cache = cache.clone();

        tokio::spawn(async move {
            match socket.set_nodelay(true) {
                Ok(n) => n,
                Err(e) => eprintln!("Failed to set nodelay on socket, error: {}", e),
            }

            js5::read_revision(revision, &mut socket, &mut cache).await;
        });
    }
}
