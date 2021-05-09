use crate::Error;

pub fn xor(a: &[u8], b: &[u8]) -> Result<Vec<u8>, Error> {
    if a.len() != b.len() {
        return Err(Error::Generic("a and b is not equal in length"))
    }

    Ok(a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect())
}

pub fn xor_char(a: &[char], b: u32) -> Result<Vec<char>, Error> {
    Ok(a.iter().map(|a| std::char::from_u32((*a as u32) ^ b).unwrap()).collect())
}

pub fn xor_repeat(a: &[u8], b: &[u8]) -> Result<Vec<u8>, Error> {
    let mut result = Vec::new();
    result.resize(a.len(), 0);
    for i in 0..a.len() {
        result[i] = a[i] ^ b[i % b.len()];
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::xor::{xor, xor_char, xor_repeat};
    use std::char::from_u32;

    #[test]
    fn xor_test() {
        assert_eq!(vec![92, 25, 41], xor(&vec![127, 93, 111], &vec![35, 68, 70]).unwrap());
    }

    #[test]
    fn xor_repeat_test() {
        assert_eq!(vec![92, 25, 41], xor_repeat(&vec![127, 93, 111, 93, 111],
                                                &vec![35, 68, 70]).unwrap());
    }

    #[test]
    fn xor_char_test() {
        assert_eq!(vec![from_u32(57).unwrap(), from_u32(27).unwrap(), from_u32(41).unwrap()], xor_char(&vec![from_u32(127).unwrap(), from_u32(93).unwrap(), from_u32(111).unwrap()], 70).unwrap());
    }
}