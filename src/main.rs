pub mod tcp;
use std::io;
use tcp::tcp_server;

#[tokio::main]
async fn main() -> io::Result<()>{
    tcp_server().await
}