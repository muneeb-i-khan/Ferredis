use crate::{
    resp::{Array, BulkString, Encodings, SimpleString},
    tcp::TCPBuffer,
};
use std::str::from_utf8;
use crate::cmd;

fn classify(command: &str) -> (char, String) {
    let mut cmd = command.chars();
    let cmd_type: char = cmd.next().unwrap();
    (cmd_type, cmd.collect())
}

async fn parse_command(stream: TCPBuffer) -> Result<(String, Vec<String>), &'static str> {
    let init_command = from_utf8(&stream).map_err(|_| "Could not convert stream")?;
    let (cmd_type, cmd) = classify(init_command);
    match cmd_type {
        '+' => Ok((SimpleString::decode(cmd).to_lowercase(), Vec::new())),
        '$' => Ok((BulkString::decode(cmd).to_lowercase(), Vec::new())),
        '*' => {
            let mut args = Array::decode(cmd);
            let command_name = args.remove(0).to_lowercase();
            Ok((command_name, args))
        }
        _ => Err("Invalid Command type"),
    }
}

pub async fn run_cmd(buffer: TCPBuffer) -> Vec<u8> {
    match parse_command(buffer).await {
        Ok((cmd_name, args)) => match cmd_name.as_str() {
            "ping" => SimpleString::encode("PONG".into()),
            "echo" => {
                let str = args.get(0).unwrap();
                BulkString::encode(str.into())
            },
            "set" => {
                if let (Some(key), Some(val)) = (args.get(0), args.get(1)) {
                    cmd::set(key, val).await;
                    SimpleString::encode("OK".to_string())
                } else {
                    SimpleString::encode("Error: Missing key or value".to_string())
                }
            }
            "get" => {
                if let Some(key) = args.get(0) {
                    match cmd::get(key).await {
                        Some(val) => BulkString::encode(val),
                        None => SimpleString::encode("nil".to_string()),
                    }
                } else {
                    SimpleString::encode("Error: Missing key".to_string())
                }
            }
            _ => SimpleString::encode("Invalid Command name".to_string()),
        },
        Err(err) => SimpleString::encode(err.to_string()),
    }
}
