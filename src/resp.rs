pub const CR: &str = "\r";
pub const LF: &str = "\n";
pub const CRLF: &str = "\r\n";

pub trait Encodings<T> {
    fn encode(data: T) -> Vec<u8>;
    fn decode(endoded_data: String) -> T;
}

pub struct SimpleString;
impl Encodings<String> for SimpleString {
    fn encode(data: String) -> Vec<u8> {
        format!("+{}\r\n", data).into_bytes()
    }

    fn decode(data: String) -> String {
        let mut tokens = data.split(CRLF);
        tokens
            .next()
            .expect("Could not parse simple string")
            .to_string()
    }
}

pub struct BulkString;

impl Encodings<String> for BulkString {
    fn encode(data: String) -> Vec<u8> {
        format!("${}\r\n{}\r\n", data.len(), data).into_bytes()
    }

    fn decode(data: String) -> String {
        let mut tokens = data.split(CRLF);
        tokens.next();
        tokens.next().expect("Could not parse bulk string").into()
    }
}

pub struct Array;
impl Encodings<Vec<String>> for Array {
    fn encode(_data: Vec<String>) -> Vec<u8> {
        unimplemented!()
    }

    fn decode(data: String) -> Vec<String> {
        let mut array = Vec::new();
        let mut chars = data.chars();

        while let Some(ch) = chars.next() {
            if ch == '$' {
                let mut len_str = String::new();
                while let Some(ch) = chars.next() {
                    if ch == '\r' {
                        chars.next();
                        break;
                    }
                    len_str.push(ch);
                }

                let len = len_str
                    .parse::<usize>()
                    .expect("Could not parse bulk string length");
                let mut bulk_string = String::with_capacity(len);

                for _ in 0..len {
                    bulk_string.push(chars.next().expect("Unexpected end of bulk string"));
                }

                chars.next();
                chars.next();

                array.push(bulk_string);
            }
        }
        array
    }
}
