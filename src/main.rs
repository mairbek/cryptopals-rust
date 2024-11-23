use aes::cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit};
use aes::Aes128;
use base64::prelude::*;
use std::collections::HashSet;
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
    Challenge6 {
        #[structopt(long)]
        file: String,
    },
    Challenge7 {
        #[structopt(long)]
        file: String,
        #[structopt(long)]
        key: String,
    },
    Challenge8 {
        #[structopt(long)]
        file: String,
    },
    Challenge9 {
        #[structopt(long)]
        unpadded: String,
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

fn hamming(a: &[u8], b: &[u8]) -> u8 {
    let n = a.len();
    let mut result: u8 = 0;
    for i in 0..n {
        let mut xor = a[i] ^ b[i];
        while xor != 0 {
            result += xor & 1;
            xor = xor >> 1;
        }
    }
    return result;
}

fn count_repeated_blocks(a: &[u8], block_size: usize) -> usize {
    let mut set = HashSet::new();
    let mut repeated = 0;
    for chunk in a.chunks(block_size) {
        if !set.insert(chunk) {
            repeated += 1;
        }
    }
    return repeated;
}

fn to_transposed_2d_array(input: &[u8], block_size: usize) -> Vec<Vec<u8>> {
    let row_count = input.len() / block_size; // Calculate the number of rows
    let mut transposed = vec![vec![0u8; row_count]; block_size]; // Preallocate transposed array

    for (i, &val) in input.iter().enumerate() {
        if i >= row_count * block_size {
            break;
        }
        let row = i / block_size;
        let col = i % block_size;
        transposed[col][row] = val; // Assign directly to the transposed position
    }

    transposed
}

fn pkcs7_pad(unpadded: &[u8], block_size: usize) -> Vec<u8> {
    let padding = block_size - (unpadded.len() % block_size);
    let mut padded = Vec::from(unpadded);
    for _ in 0..padding {
        padded.push(padding as u8);
    }
    padded
}
fn print_bytes(bytes: &[u8]) -> String {
    use std::ascii::escape_default;

    bytes
        .iter()
        .flat_map(|&b| escape_default(b))
        .map(|b| b as char)
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
        Cli::Challenge6 { file } => {
            let contents = std::fs::read_to_string(file).unwrap();
            let content_decoded = BASE64_STANDARD.decode(contents.replace("\n", "")).unwrap();
            let mut scores: Vec<(u8, f32)> = Vec::new();
            for key_size in 2..=40 {
                let mut score = 0.0;
                let num_blocks = 4;
                for i in 0..num_blocks {
                    let a = &content_decoded[i * key_size..(i + 1) * key_size];
                    let b = &content_decoded[(i + 1) * key_size..(i + 2) * key_size];
                    score += (hamming(a, b) as f32) / (key_size as f32);
                }
                score /= num_blocks as f32;
                scores.push((key_size as u8, score));
            }
            scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

            let mut key_scores: Vec<(Vec<u8>, f64)> = Vec::new();
            for i in 0..4 {
                let (m, _score) = scores[i];
                let min_key_size = m as usize;
                let t = to_transposed_2d_array(&content_decoded, min_key_size);
                let mut keys: Vec<u8> = Vec::new();
                let mut total_score = 0.0;
                for i in 0..min_key_size {
                    let (key, score) = crack_single_byte_xor(&t[i], 0, 0xfe);
                    keys.push(key);
                    total_score += score;
                }
                key_scores.push((keys, total_score / min_key_size as f64));
            }
            key_scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            let v = xor_repeatedkey(&content_decoded, &key_scores[0].0);
            println!("{}", String::from_utf8(v).unwrap());
        }
        Cli::Challenge7 { file, key } => {
            let cipher = Aes128::new(GenericArray::from_slice(key.as_bytes()));
            let contents = std::fs::read_to_string(file).unwrap();
            let content_decoded = BASE64_STANDARD.decode(contents.replace("\n", "")).unwrap();
            let mut result: Vec<u8> = Vec::new();
            for i in 1..content_decoded.len() / 16 {
                let mut block =
                    GenericArray::clone_from_slice(&content_decoded[i * 16..(i + 1) * 16]);
                cipher.decrypt_block(&mut block);
                result.extend(&block);
            }
            println!("{}", String::from_utf8_lossy(&result));
        }
        Cli::Challenge8 { file } => {
            let contents = std::fs::read_to_string(file).unwrap();
            for line in contents.lines() {
                let count = count_repeated_blocks(&line.as_bytes(), 16);
                if count > 0 {
                    println!("{} {}", line, count);
                }
            }
        }
        Cli::Challenge9 { unpadded } => {
            let padded = pkcs7_pad(unpadded.as_bytes(), 20);
            println!("{}", print_bytes(&padded));
        }
    }
}
