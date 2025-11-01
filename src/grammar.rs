macro_rules! morse_alphabet {
    ($name:ident {
        $($val:ident => $char:expr => $str:expr),*

    }
    ) => {
        pub enum $name {
            $($val),*
        }
        impl $name {
            pub const fn to_morse(&self) -> &'static str {
                match self {
                    $($name::$val => $str),*
                }
            }
            pub fn to_char(&self) -> char {
                match self {
                    $($name::$val => $char),*
                }
            }
            pub fn all() -> Vec<$name> {
                vec![
                    $($name::$val),*
                ]
            }
            pub fn map() -> std::collections::HashMap<char, &'static str> {
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
