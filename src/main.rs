use base64::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt)]
enum Cli {
    Chalenge1 {
        #[structopt(short, long)]
        input: String,
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

fn main() {
    let args = Cli::from_args();
    match args {
        Cli::Chalenge1 { input } => {
            let bytes = decode_hex(&input);
            println!("{}", BASE64_STANDARD.encode(&bytes));
        }
    }
}
