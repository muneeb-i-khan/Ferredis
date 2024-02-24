use std::error::Error;
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

pub async fn tcp_server() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    println!("Server listening on port 6379...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {:?}", stream.peer_addr());
                handle_connection(stream).await.unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}

async fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let res = "+PONG\r\n";

    loop {
        let mut buf = [0; 512];
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                println!("RES: {:?}", res.as_bytes());
                stream.write(res.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("Error occured: {}", e);
                break;
            }
        }
    }
    Ok(())
}
