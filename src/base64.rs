use crate::Error;

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

#[cfg(test)]
mod tests {
    use crate::base64::to_base64;

    #[test]
    fn to_base64_test() {
        assert_eq!("AQID", to_base64(&[1, 2, 3]).unwrap());
    }

    #[test]
    fn to_base64_byte_3_to_5() {
        assert_eq!("IGtp", to_base64(&[32, 107, 105]).unwrap());
    }
}