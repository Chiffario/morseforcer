use std::collections::HashMap;

use clap::{Parser, arg};

macro_rules! morse_alphabet {
    ($name:ident {
        $($val:ident => $char:expr => $str:expr),*

    }
    ) => {
        enum $name {
            $($val),*
        }
        impl $name {
            const fn to_morse(&self) -> &'static str {
                match self {
                    $($name::$val => $str),*
                }
            }
            fn to_char(&self) -> char {
                match self {
                    $($name::$val => $char),*
                }
            }
            fn all() -> Vec<$name> {
                vec![
                    $($name::$val),*
                ]
            }
            fn map() -> HashMap<char, &'static str> {
                $name::all().into_iter().map(|i| (i.to_char(), i.to_morse())).collect()
            }
        }
    }
}
morse_alphabet!(English {
    A => 'a' => ".-",
    B => 'b' => "-...",
    C => 'c' => "-.-.",
    D => 'd' => "-..",
    E => 'e' => ".",
    F => 'f' => "..-.",
    G => 'g' => "--.",
    H => 'h' => "....",
    I => 'i' => "..",
    J => 'j' => ".---",
    K => 'k' => "-.-",
    L => 'l' => ".-..",
    M => 'm' => "--",
    N => 'n' => "-.",
    O => 'o' => "---",
    P => 'p' => ".--.",
    Q => 'q' => "--.-",
    R => 'r' => ".-.",
    S => 's' => "...",
    T => 't' => "-",
    U => 'u' => "..-",
    V => 'v' => "...-",
    W => 'w' => ".--",
    X => 'x' => "-..-",
    Y => 'y' => "-.--",
    Z => 'z' => "--..",
    Number1 => '1' => ".----",
    Number2 => '2' => "..---",
    Number3 => '3' => "...--",
    Number4 => '4' => "....-",
    Number5 => '5' => ".....",
    Number6 => '6' => "-....",
    Number7 => '7' => "--...",
    Number8 => '8' => "---..",
    Number9 => '9' => "----.",
    Number0 => '0' => "-----",
    Underscore => '_' => "..--.-"
});

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
    let encoding = English::map();
    let mut path = Vec::new();

    decode_morse(&morse_code, args.limit, &mut path, &encoding);
}
