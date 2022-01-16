use std::env;
use std::error::Error;
use tokio::io;
use tokio::io::AsyncReadExt;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener, TcpStream};

const WORLD: u8 = 14;
const JS5: u8 = 15;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listen_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:43594".to_string());
    let world_addr = env::args()
        .nth(2)
        .unwrap_or_else(|| "127.0.0.1:43595".to_string());
    let js5_addr = env::args()
        .nth(2)
        .unwrap_or_else(|| "127.0.0.1:43596".to_string());

    println!("Listening on: {}", listen_addr);
    println!("Proxying to:");
    println!("World: {}", world_addr);
    println!("JS5: {}", js5_addr);

    let listener = TcpListener::bind(listen_addr).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        let world_addr = world_addr.clone();
        let js5_addr = js5_addr.clone();

        tokio::spawn(async move {
            match socket.set_nodelay(true) {
                Ok(n) => n,
                Err(e) => eprintln!("Failed to set nodelay on socket, error: {}", e),
            }

            if let Ok(service) = socket.read_u8().await {
                match service {
                    WORLD => transfer(socket, world_addr).await.unwrap(),
                    JS5 => transfer(socket, js5_addr).await.unwrap(),
                    _ => (),
                }
            }
        });
    }
}

async fn transfer(inbound: TcpStream, proxy_addr: String) -> Result<(), Box<dyn Error>> {
    let outbound = TcpStream::connect(proxy_addr).await?;

    let (ri, wi) = inbound.into_split();
    let (ro, wo) = outbound.into_split();

    create_proxy_task(ri, wo);
    create_proxy_task(ro, wi);

    Ok(())
}

fn create_proxy_task(mut reader: OwnedReadHalf, mut writer: OwnedWriteHalf) {
    tokio::spawn(async move {
        loop {
            if let Ok(copied_bytes) = io::copy(&mut reader, &mut writer).await {
                if copied_bytes == 0 {
                    break;
                }
            } else {
                break;
            }
        }
    });
}
