mod grammar;

use std::collections::HashMap;

use clap::{Parser, arg};

fn decode_morse(morse: &str, length: u32, path: &mut Vec<char>, encoding: &HashMap<char, &str>) {
    if length == 0 && morse.is_empty() {
        println!("{}", path.iter().collect::<String>());
        return;
    } else if length == 0 || morse.is_empty() {
        return;
    }

    for (&ch, &code) in encoding.iter() {
        if morse.starts_with(code) {
            path.push(ch);
            decode_morse(&morse[code.len()..], length - 1, path, encoding);
            path.pop();
        }
    }
}

#[derive(Parser)]
struct Args {
    pub morse: String,

    #[arg(short, long)]
    pub limit: u32,
}

fn main() {
    let args = Args::parse();
    let morse_code = args.morse;
    let encoding = grammar::English::map();
    let mut path = Vec::new();

    decode_morse(&morse_code, args.limit, &mut path, &encoding);
}
