use crate::{
    resp::{Array, BulkString, Encodings, SimpleString},
    tcp::TCPBuffer,
};
use std::str::from_utf8;

fn classify(command: &str) -> (char, String) {
    let mut cmd = command.chars();
    let cmd_type: char = cmd.next().unwrap();
    (cmd_type, cmd.collect())
}

fn parse_command(stream: TCPBuffer) -> (String, Vec<String>) {
    let init_command = from_utf8(&stream).expect("Could not convert stream");
    let (cmd_type, cmd) = classify(init_command);
    match cmd_type {
        '+' => (SimpleString::decode(cmd).to_lowercase(), Vec::new()),
        '$' => (BulkString::decode(cmd).to_lowercase(), Vec::new()),
        '*' => {
            let mut args = Array::decode(cmd);
            let command_name = args.remove(0).to_lowercase();
            (command_name, args)
        }
        _ => {
            panic!("Invalid Command type");
        }
    }
}

pub fn run_cmd(buffer: TCPBuffer) -> Vec<u8> {
    let (cmd_name, args) = parse_command(buffer);

    match cmd_name.as_str() {
        "ping" => SimpleString::encode("PONG".into()),
        "echo" => {
            let str = args.get(0).unwrap();
            BulkString::encode(str.into())
        }
        _ => {
            panic!("Invalid Command name");
        }
    }
}
