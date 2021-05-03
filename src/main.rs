use crate::xor::xor_char;
use std::collections::HashMap;

mod hex;
mod base64;
mod xor;

#[derive(Debug)]
pub enum Error {
    Generic(&'static str),
    GenericStr(String),
    Utf8Error(std::str::Utf8Error),
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Error {
        Error::Utf8Error(err)
    }
}

fn english_char_percentage(c: char) -> f64 {
    match c {
        'a' => 8.167,
        'b' => 1.492,
        'c' => 2.782,
        'd' => 4.253,
        'e' => 12.702,
        'f' => 2.228,
        'g' => 2.015,
        'h' => 6.094,
        'i' => 6.966,
        'j' => 0.153,
        'k' => 0.772,
        'l' => 4.025,
        'm' => 2.406,
        'n' => 6.749,
        'o' => 7.507,
        'p' => 1.929,
        'q' => 0.095,
        'r' => 5.987,
        's' => 6.327,
        't' => 9.056,
        'u' => 2.758,
        'v' => 0.978,
        'w' => 2.360,
        'x' => 0.150,
        'y' => 1.974,
        'z' => 0.074,
        _ => 0.0
    }
}

fn calc_char_percentages(chars: &[char]) -> HashMap<char, f64> {
    let mut map = HashMap::new();

    for c in chars.iter() {
      map.insert(*c, *map.get(c).unwrap_or(&0.0) + 1.0);
    }

    for (_, val) in map.iter_mut() {
        *val = (*val / chars.len() as f64) * 100.0;
    }
    map
}

fn calc_percentage_diff(percentages: HashMap<char, f64>) -> f64 {
    let mut accumulated_diff = 0.0;

    for key in percentages.keys() {
        accumulated_diff += f64::abs(percentages.get(key).unwrap_or(&0.0) - english_char_percentage(*key));
    }

    accumulated_diff
}

fn guess_xor_byte(input: &str) -> String {
    let mut lowest_diff = f64::MAX;
    let mut lowest_diff_byte = 0;
    for x in 0..=255 {
        let xored = xor_char(&input.chars().collect::<Vec<char>>(), x).unwrap();

        let percentages = calc_char_percentages(&xored);

        let diff = calc_percentage_diff(percentages);

        if diff < lowest_diff {
            lowest_diff = diff;
            lowest_diff_byte = x;
        }
    }

    xor_char(&input.chars().collect::<Vec<char>>(), lowest_diff_byte).unwrap().into_iter().collect()
}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::hex::parse_hex;
    use crate::base64::to_base64;
    use crate::xor::xor;
    use crate::{guess_xor_byte, calc_char_percentages};
    use std::collections::HashMap;

    #[test]
    fn calc_char_percentages_test() {
        let mut expected = HashMap::new();
        expected.insert('a', 50.0);
        expected.insert('b', 50.0);
        assert_eq!(expected,
                   calc_char_percentages(&vec!['a', 'a', 'b', 'b']));

    }

    #[test]
    fn crypto_pals_challenge1_hex() {
        assert_eq!(vec![73, 39, 109, 32, 107, 105, 108, 108, 105, 110, 103, 32, 121, 111, 117, 114, 32, 98, 114, 97, 105, 110, 32, 108, 105, 107, 101, 32, 97, 32, 112, 111, 105, 115, 111, 110, 111, 117, 115, 32, 109, 117, 115, 104, 114, 111, 111, 109],
                   parse_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap());
    }

    #[test]
    fn crypto_pals_challenge1_complete() {
        assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
                   to_base64(&parse_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap()).unwrap());
    }

    #[test]
    fn crypto_pals_challenge2_complete() {
        assert_eq!(parse_hex("746865206b696420646f6e277420706c6179").unwrap(),
                   xor(&parse_hex("1c0111001f010100061a024b53535009181c").unwrap(), &parse_hex("686974207468652062756c6c277320657965").unwrap()).unwrap());
    }

    #[test]
    fn crypto_pals_challenge3_complete() {
        assert_eq!("Cooking MC's like a pound of bacon",
                   guess_xor_byte(&std::str::from_utf8(&parse_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap()).unwrap()));
    }
}