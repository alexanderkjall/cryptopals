use crate::Error;

fn map_from_base64(c: u8) -> Result<u8, Error> {
    match c {
        b'A' => Ok(0b00000000),
        b'B' => Ok(0b00000001),
        b'C' => Ok(0b00000010),
        b'D' => Ok(0b00000011),
        b'E' => Ok(0b00000100),
        b'F' => Ok(0b00000101),
        b'G' => Ok(0b00000110),
        b'H' => Ok(0b00000111),
        b'I' => Ok(0b00001000),
        b'J' => Ok(0b00001001),
        b'K' => Ok(0b00001010),
        b'L' => Ok(0b00001011),
        b'M' => Ok(0b00001100),
        b'N' => Ok(0b00001101),
        b'O' => Ok(0b00001110),
        b'P' => Ok(0b00001111),
        b'Q' => Ok(0b00010000),
        b'R' => Ok(0b00010001),
        b'S' => Ok(0b00010010),
        b'T' => Ok(0b00010011),
        b'U' => Ok(0b00010100),
        b'V' => Ok(0b00010101),
        b'W' => Ok(0b00010110),
        b'X' => Ok(0b00010111),
        b'Y' => Ok(0b00011000),
        b'Z' => Ok(0b00011001),
        b'a' => Ok(0b00011010),
        b'b' => Ok(0b00011011),
        b'c' => Ok(0b00011100),
        b'd' => Ok(0b00011101),
        b'e' => Ok(0b00011110),
        b'f' => Ok(0b00011111),
        b'g' => Ok(0b00100000),
        b'h' => Ok(0b00100001),
        b'i' => Ok(0b00100010),
        b'j' => Ok(0b00100011),
        b'k' => Ok(0b00100100),
        b'l' => Ok(0b00100101),
        b'm' => Ok(0b00100110),
        b'n' => Ok(0b00100111),
        b'o' => Ok(0b00101000),
        b'p' => Ok(0b00101001),
        b'q' => Ok(0b00101010),
        b'r' => Ok(0b00101011),
        b's' => Ok(0b00101100),
        b't' => Ok(0b00101101),
        b'u' => Ok(0b00101110),
        b'v' => Ok(0b00101111),
        b'w' => Ok(0b00110000),
        b'x' => Ok(0b00110001),
        b'y' => Ok(0b00110010),
        b'z' => Ok(0b00110011),
        b'0' => Ok(0b00110100),
        b'1' => Ok(0b00110101),
        b'2' => Ok(0b00110110),
        b'3' => Ok(0b00110111),
        b'4' => Ok(0b00111000),
        b'5' => Ok(0b00111001),
        b'6' => Ok(0b00111010),
        b'7' => Ok(0b00111011),
        b'8' => Ok(0b00111100),
        b'9' => Ok(0b00111101),
        b'+' => Ok(0b00111110),
        b'/' => Ok(0b00111111),
        _ => Err(Error::GenericStr(format!("not a base64 char {}", c)))
    }
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
    Ok([map_base64(b1 >> 2)?,
        map_base64((b1 & 0b00000011) << 4 | (b2 & 0b11110000) >> 4)?,
        map_base64((b2 & 0b00001111) << 2 | (b3 & 0b11000000) >> 6)?,
        map_base64(b3 & 0b00111111)?])
}

fn remainder_2(b1: u8, b2: u8) -> Result<[u8; 4], Error> {
    Ok([map_base64(b1 >> 2)?,
        map_base64((b1 & 0b00000011) << 4 | (b2 & 0b11110000) >> 4)?,
        map_base64((b2 & 0b00001111) << 2)?,
        b'='])
}

fn remainder_1(b1: u8) -> Result<[u8; 4], Error> {
    Ok([map_base64(b1 >> 2)?,
        map_base64((b1 & 0b00000011) << 4)?,
        b'=',
        b'='])
}

pub fn to_base64(data: &[u8]) -> Result<String, Error> {
    let mut output: Vec<u8> = vec![];

    for i in 0..((data.len() + 2) / 3) {
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


fn pack_bits(b1: u8, b2: u8, b3: u8, b4: u8) -> Result<[u8; 3], Error> {
    let b1 = map_from_base64(b1)?;
    let b2 = map_from_base64(b2)?;
    let b3 = map_from_base64(b3)?;
    let b4 = map_from_base64(b4)?;

    Ok([(b1 << 2) | (b2 >> 4),
        (b2 << 4) | (b3 >> 2),
        (b3 << 6) | b4])
}

fn pack_remainder_2(b1: u8, b2: u8, b3: u8) -> Result<[u8; 2], Error> {
    let b1 = map_from_base64(b1)?;
    let b2 = map_from_base64(b2)?;
    let b3 = map_from_base64(b3)?;

    Ok([(b1 << 2) | (b2 >> 4),
        (b2 << 4) | (b3 >> 2)])
}

fn pack_remainder_1(b1: u8, b2: u8) -> Result<u8, Error> {
    let b1 = map_from_base64(b1)?;
    let b2 = map_from_base64(b2)?;

    Ok((b1 << 2) | (b2 >> 4))
}

pub fn from_base64(data: &str) -> Result<Vec<u8>, Error> {
    let data = data.as_bytes();
    if data.len() % 4 != 0 {
        return Err(Error::Generic("data isn't a multiple of 4"))
    }

    let mut output: Vec<u8> = vec![];

    for i in 0..((data.len() + 3) / 4) {
        if data[i * 4 + 2] == b'=' && data[i * 4 + 3] == b'=' {
            output.push(pack_remainder_1(data[i * 4], data[i * 4 + 1])?);
        } else if data[i * 4 + 3] == b'=' {
            output.extend(&pack_remainder_2(data[i * 4], data[i * 4 + 1], data[i * 4 + 2])?);
        } else {
            output.extend(&pack_bits(data[i * 4], data[i * 4 + 1], data[i * 4 + 2], data[i * 4 + 3])?);
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use crate::base64::{to_base64, from_base64};

    #[test]
    fn roundtrip_1() {
        assert_eq!(vec![1], from_base64(&to_base64(&[1]).unwrap()).unwrap());
    }

    #[test]
    fn roundtrip_2() {
        assert_eq!(vec![1, 2], from_base64(&to_base64(&[1, 2]).unwrap()).unwrap());
    }

    #[test]
    fn roundtrip_3() {
        assert_eq!(vec![1, 2, 3], from_base64(&to_base64(&[1, 2, 3]).unwrap()).unwrap());
    }

    #[test]
    fn from_base64_test() {
        assert_eq!(vec![1, 2, 3], from_base64("AQID").unwrap());
    }

    #[test]
    fn to_base64_test() {
        assert_eq!("AQID", to_base64(&[1, 2, 3]).unwrap());
    }

    #[test]
    fn to_base64_byte_3_to_5() {
        assert_eq!("IGtp", to_base64(&[32, 107, 105]).unwrap());
    }
}