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


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::hex::parse_hex;
    use crate::base64::to_base64;
    use crate::xor::xor;

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