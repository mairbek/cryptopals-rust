use base64::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt)]
enum Cli {
    Chalenge1 {
        #[structopt(long)]
        input: String,
    },
    Challenge2 {
        #[structopt(long)]
        input1: String,
        #[structopt(long)]
        input2: String,
    },
}

fn decode_hex(hex: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut chars = hex.chars().peekable();
    while let Some(c) = chars.next() {
        let hi = c.to_digit(16).unwrap();
        let lo = chars.next().unwrap().to_digit(16).unwrap();
        bytes.push((hi << 4 | lo) as u8);
    }
    bytes
}

fn encode_hex(bytes: &[u8]) -> String {
    let mut hex = String::new();
    for byte in bytes {
        hex.push_str(&format!("{:02x}", byte));
    }
    hex
}

fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect()
}

fn main() {
    let args = Cli::from_args();
    match args {
        Cli::Chalenge1 { input } => {
            let bytes = decode_hex(&input);
            println!("{}", BASE64_STANDARD.encode(&bytes));
        }
        Cli::Challenge2 { input1, input2 } => {
            let bytes1 = decode_hex(&input1);
            let bytes2 = decode_hex(&input2);
            let xor = xor_bytes(&bytes1, &bytes2);
            println!("{}", encode_hex(&xor));
        }
    }
}
