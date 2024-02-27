use std::error::Error;
use std::io;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::parser::run_cmd;

const BUFFER_SIZE: usize = 1024;
pub type TCPBuffer = [u8; BUFFER_SIZE];
const PORT: &str = "127.0.0.1:6379";

pub async fn tcp_server() -> io::Result<()> {
    let listener = TcpListener::bind(PORT).await.unwrap();
    println!("Server listening on port 6379...");

    loop {
        let stream_res = listener.accept().await;
        match stream_res {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(stream).await {
                        println!("Error occurred: {}", e);
                    }
                });
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    loop {
        let mut buf = [0; 1024];
        match stream.read(&mut buf).await {
            Ok(0) => break,
            Ok(_) => {
                let res = run_cmd(buf).await;
                stream.write_all(res.as_slice()).await.unwrap();
            }
            Err(e) => {
                println!("Error occurred: {}", e);
                break;
            }
        }
    }

    Ok(())
}
