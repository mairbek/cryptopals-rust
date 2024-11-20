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
    Challenge3 {
        #[structopt(long)]
        input: String,
    },
    Challenge4 {
        #[structopt(long)]
        file: String,
    },
    Challenge5 {
        #[structopt(long)]
        file: String,
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

fn english_score(x: &[u8]) -> f64 {
    let letter_distribution = [
        8.167,  // a
        1.492,  // b
        2.782,  // c
        4.253,  // d
        12.702, // e
        2.228,  // f
        2.015,  // g
        6.094,  // h
        6.966,  // i
        0.153,  // j
        0.772,  // k
        4.025,  // l
        2.406,  // m
        6.749,  // n
        7.507,  // o
        1.929,  // p
        0.095,  // q
        5.987,  // r
        6.327,  // s
        9.056,  // t
        2.758,  // u
        0.978,  // v
        2.360,  // w
        0.150,  // x
        1.974,  // y
        0.074,  // z
    ];
    let mut score = 0.0;
    for &byte in x {
        if byte >= b'a' && byte <= b'z' {
            score += letter_distribution[(byte - b'a') as usize];
        }
        if byte >= b'A' && byte <= b'Z' {
            letter_distribution[(byte - b'A') as usize];
        }
        if byte == b' ' {
            score += 13.0;
        }
    }
    score
}

fn crack_single_byte_xor(input: &[u8], from: u8, to: u8) -> (u8, f64) {
    let mut best_key = from;
    let mut best_score = 0.0;
    for key in from..=to {
        let xored = input.iter().map(|byte| byte ^ key).collect::<Vec<u8>>();
        let score = english_score(&xored);
        if score > best_score {
            best_score = score;
            best_key = key;
        }
    }
    (best_key, best_score)
}

fn xor_single_byte(input: &[u8], key: u8) -> Vec<u8> {
    input.iter().map(|byte| byte ^ key).collect()
}

fn xor_repeatedkey(a: &[u8], key: &[u8]) -> Vec<u8> {
    a.iter()
        .zip(key.iter().cycle())
        .map(|(x, y)| x ^ y)
        .collect()
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
        Cli::Challenge3 { input } => {
            let bytes = decode_hex(&input);
            let (key, _score) = crack_single_byte_xor(&bytes, b'A', b'Z');
            println!("{}", key as char);
            println!(
                "{}",
                String::from_utf8(xor_single_byte(&bytes, key)).unwrap()
            );
        }
        Cli::Challenge4 { file } => {
            let contents = std::fs::read_to_string(file).unwrap();
            let mut best_score = 0.0;
            let mut best_key = 0;
            let mut best_line = String::new();
            for line in contents.lines() {
                let bytes = decode_hex(line);
                let (key, score) = crack_single_byte_xor(&bytes, 0, 0xfe);
                if score > best_score {
                    best_score = score;
                    best_key = key;
                    best_line = line.to_string();
                }
            }
            println!("{}", best_key as char);
            println!("{}", best_line);
            println!(
                "{}",
                String::from_utf8(xor_single_byte(&decode_hex(&best_line), best_key)).unwrap()
            );
        }
        Cli::Challenge5 { file } => {
            let contents = std::fs::read_to_string(file).unwrap();
            let xor = xor_repeatedkey(contents.as_bytes(), "ICE".as_bytes());
            println!("{}", encode_hex(&xor));
        }
    }
}
