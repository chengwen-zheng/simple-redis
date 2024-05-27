use anyhow::Result;
use simple_redis::Backend;
use tokio::net::TcpListener;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:6379";
    info!("Simple redis is listening on: {}", addr);
    let listener = TcpListener::bind(addr).await?;

    let backend = Backend::new();
    loop {
        let (socket, raddr) = listener.accept().await?;
        let clone_backend = backend.clone();
        info!("new connection from: {:?}", raddr);
        tokio::spawn(async move {
            match simple_redis::network::stream_handler(socket, clone_backend).await {
                Ok(_) => info!("Connection from {:?} closed", raddr),
                Err(e) => warn!("Connection raddr:{:?}: error: {:?}", raddr, e),
            }
        });
    }
}
