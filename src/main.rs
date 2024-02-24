pub mod tcp;
use tcp::tcp_server;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tcp_server().await
}