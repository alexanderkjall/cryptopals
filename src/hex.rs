use crate::Error;

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


#[cfg(test)]
mod tests {
    use crate::hex::parse_hex;

    #[test]
    fn parse_hex_test() {
        assert_eq!(vec![1, 2, 3], parse_hex("010203").unwrap());
    }

    #[test]
    fn parse_hex_uneven_test() {
        assert_eq!(vec![1, 2, 3], parse_hex("10203").unwrap());
    }
}