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

fn val(c: u8, idx: usize) -> Result<u8, Error> {
    match c {
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'0'..=b'9' => Ok(c - b'0'),
        _ => Err(Error::GenericStr(format!("illegal character {} at {}", c, idx))),
    }
}

pub fn parse_hex(hex: &str) -> Result<Vec<u8>, Error> {
    let hex = if hex.len() % 2 != 0 {
        format!("0{}", hex)
    } else {
        hex.to_owned()
    };

    hex.as_bytes()
        .chunks(2)
        .enumerate()
        .map(|(i, pair)| Ok(val(pair[0], 2 * i)? << 4 | val(pair[1], 2 * i + 1)?))
        .collect()
}

fn map_base64(c: u8) -> Result<u8, Error> {
    match c {
        0b00000000 => Ok(b'A'),
        0b00000001 => Ok(b'B'),
        0b00000010 => Ok(b'C'),
        0b00000011 => Ok(b'D'),
        0b00000100 => Ok(b'E'),
        0b00000101 => Ok(b'F'),
        0b00000110 => Ok(b'G'),
        0b00000111 => Ok(b'H'),
        0b00001000 => Ok(b'I'),
        0b00001001 => Ok(b'J'),
        0b00001010 => Ok(b'K'),
        0b00001011 => Ok(b'L'),
        0b00001100 => Ok(b'M'),
        0b00001101 => Ok(b'N'),
        0b00001110 => Ok(b'O'),
        0b00001111 => Ok(b'P'),
        0b00010000 => Ok(b'Q'),
        0b00010001 => Ok(b'R'),
        0b00010010 => Ok(b'S'),
        0b00010011 => Ok(b'T'),
        0b00010100 => Ok(b'U'),
        0b00010101 => Ok(b'V'),
        0b00010110 => Ok(b'W'),
        0b00010111 => Ok(b'X'),
        0b00011000 => Ok(b'Y'),
        0b00011001 => Ok(b'Z'),
        0b00011010 => Ok(b'a'),
        0b00011011 => Ok(b'b'),
        0b00011100 => Ok(b'c'),
        0b00011101 => Ok(b'd'),
        0b00011110 => Ok(b'e'),
        0b00011111 => Ok(b'f'),
        0b00100000 => Ok(b'g'),
        0b00100001 => Ok(b'h'),
        0b00100010 => Ok(b'i'),
        0b00100011 => Ok(b'j'),
        0b00100100 => Ok(b'k'),
        0b00100101 => Ok(b'l'),
        0b00100110 => Ok(b'm'),
        0b00100111 => Ok(b'n'),
        0b00101000 => Ok(b'o'),
        0b00101001 => Ok(b'p'),
        0b00101010 => Ok(b'q'),
        0b00101011 => Ok(b'r'),
        0b00101100 => Ok(b's'),
        0b00101101 => Ok(b't'),
        0b00101110 => Ok(b'u'),
        0b00101111 => Ok(b'v'),
        0b00110000 => Ok(b'w'),
        0b00110001 => Ok(b'x'),
        0b00110010 => Ok(b'y'),
        0b00110011 => Ok(b'z'),
        0b00110100 => Ok(b'0'),
        0b00110101 => Ok(b'1'),
        0b00110110 => Ok(b'2'),
        0b00110111 => Ok(b'3'),
        0b00111000 => Ok(b'4'),
        0b00111001 => Ok(b'5'),
        0b00111010 => Ok(b'6'),
        0b00111011 => Ok(b'7'),
        0b00111100 => Ok(b'8'),
        0b00111101 => Ok(b'9'),
        0b00111110 => Ok(b'+'),
        0b00111111 => Ok(b'/'),
        _ => Err(Error::GenericStr(format!("illegal bit pattern {}", c)))
    }
}

fn extract_bits(b1: u8, b2: u8, b3: u8) -> Result<[u8; 4], Error> {
    Ok([map_base64(b1.clone() >> 2)?,
        map_base64((b1 & 0b00000011) << 4 | (b2.clone() & 0b11110000) >> 4)?,
        map_base64((b2 & 0b00001111) << 2 | (b3.clone() & 0b11000000) >> 6)?,
        map_base64(b3 & 0b00111111)?])
}

fn remainder_2(b1: u8, b2: u8) -> Result<[u8; 4], Error> {
    Ok([map_base64(b1.clone() >> 2)?,
        map_base64((b1 & 0b00000011) << 4 | (b2.clone() & 0b11110000) >> 4)?,
        map_base64((b2 & 0b00001111) << 2)?,
        b'='])
}

fn remainder_1(b1: u8) -> Result<[u8; 4], Error> {
    Ok([map_base64(b1.clone() >> 2)?,
        map_base64((b1 & 0b00000011) << 4)?,
        b'=',
        b'='])
}

pub fn to_base64(data: &[u8]) -> Result<String, Error> {
    let mut output: Vec<u8> = vec![];

    for i in 0..(data.len() / 3) {
        if data.len() > i * 3 + 2 {
            output.extend(&extract_bits(data[i * 3], data[i * 3 + 1], data[i * 3 + 2])?);
        } else if data.len() > i * 3 + 1 {
            output.extend(&remainder_2(data[i * 3], data[i * 3 + 1])?);
        } else {
            output.extend(&remainder_1(data[i * 3])?);
        }
    }

    Ok(std::str::from_utf8(&output)?.to_string())
}

pub fn xor(a: &[u8], b: &[u8]) -> Result<Vec<u8>, Error> {
    if a.len() != b.len() {
        return Err(Error::Generic("a and b is not equal in length"))
    }

    Ok(a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect())
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use crate::{parse_hex, to_base64, xor};

    #[test]
    fn parse_hex_test() {
        assert_eq!(vec![1, 2, 3], parse_hex("010203").unwrap());
    }

    #[test]
    fn parse_hex_uneven_test() {
        assert_eq!(vec![1, 2, 3], parse_hex("10203").unwrap());
    }

    #[test]
    fn to_base64_test() {
        assert_eq!("AQID", to_base64(&[1, 2, 3]).unwrap());
    }

    #[test]
    fn to_base64_byte_3_to_5() {
        assert_eq!("IGtp", to_base64(&[32, 107, 105]).unwrap());
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
}