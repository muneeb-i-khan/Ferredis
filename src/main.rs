pub mod tcp;
pub mod resp;
pub mod parser;
pub mod cmd;
use std::io;
use tcp::tcp_server;

#[tokio::main]
async fn main() -> io::Result<()>{
    tcp_server().await
}