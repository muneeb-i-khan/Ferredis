use std::error::Error;
use std::io;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

pub async fn tcp_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
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
    let res = "+PONG\r\n";
    loop {
        let mut buf = [0; 512];
        match stream.read(&mut buf).await {
            Ok(0) => break,
            Ok(_) => {
                stream.write_all(res.as_bytes()).await.unwrap();
            }
            Err(e) => {
                println!("Error occured: {}", e);
                break;
            }
        }
    }

    Ok(())
}
