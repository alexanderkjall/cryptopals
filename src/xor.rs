use crate::Error;
use crate::string::{calc_char_percentages, calc_percentage_diff};
use std::collections::HashMap;

pub fn xor(a: &[u8], b: &[u8]) -> Result<Vec<u8>, Error> {
    if a.len() != b.len() {
        return Err(Error::Generic("a and b is not equal in length"))
    }

    Ok(a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect())
}

pub fn xor_char(a: &[char], b: u32) -> Result<Vec<char>, Error> {
    Ok(a.iter().map(|a| std::char::from_u32((*a as u32) ^ b).unwrap()).collect())
}

pub fn xor_repeat(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    result.resize(a.len(), 0);
    for i in 0..a.len() {
        result[i] = a[i] ^ b[i % b.len()];
    }
    result
}

pub fn guess_xor_byte(input: &[u8]) -> u8 {
    let mut lowest_diff = f64::MAX;
    let mut lowest_diff_byte = 0;
    for x in 0..=255 {
        let xored = input.iter().map(|b| (x ^ b) as char).collect::<Vec<char>>();

        let percentages = calc_char_percentages(&xored);

        let diff = calc_percentage_diff(percentages);

        if diff < lowest_diff {
            lowest_diff = diff;
            lowest_diff_byte = x;
        }
    }
    lowest_diff_byte
}

pub fn guess_xor_byte_str(input: &str) -> String {
    let lowest_diff_byte = guess_xor_byte(input.as_bytes());

    xor_char(&input.chars().collect::<Vec<char>>(), lowest_diff_byte as u32).unwrap().into_iter().collect()
}

pub fn guess_xor_byte_with_space(input: &[u8]) -> u8 {
    let input = input.iter().map(|c| c ^ b' ').collect::<Vec<u8>>();

    let mut map = HashMap::new();

    for c in input.iter() {
        map.insert(*c, *map.get(c).unwrap_or(&0) + 1);
    }

    let mut count = 0;
    let mut most_common_byte = 0;

    for (a, b) in map {
        if count < b {
            count = b;
            most_common_byte = a;
        }
    }

    most_common_byte
}

pub fn guess_xor_byte_with_space_vec(input: &[Vec<u8>]) -> Vec<u8> {
    input.iter().map(|v| {
        guess_xor_byte_with_space(&v)
    }).collect()
}

pub fn find_xored_english_string(input: &[String]) -> String {
    let mut lowest_diff = f64::MAX;
    let mut result = String::new();
    for inp in input {
        let xored = guess_xor_byte_str(inp);

        let percentages = calc_char_percentages(&xored.chars().collect::<Vec<char>>());

        let diff = calc_percentage_diff(percentages);

        if diff < lowest_diff {
            lowest_diff = diff;
            result = xored;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::xor::{xor, xor_char, xor_repeat, guess_xor_byte_with_space_vec};
    use std::char::from_u32;

    #[test]
    fn guess_xor_byte_vec_test() {
        let s = "this is my line, and i am sticking to it no matter what";

        let enc_a = xor_repeat(s.as_bytes(), &vec![b'a']);
        let enc_b = xor_repeat(s.as_bytes(), &vec![b'b']);
        let enc_c = xor_repeat(s.as_bytes(), &vec![b'c']);

        let result = guess_xor_byte_with_space_vec(&[enc_a, enc_b, enc_c]);

        assert_eq!(vec![b'a', b'b', b'c'], result);
    }

    #[test]
    fn xor_test() {
        assert_eq!(vec![92, 25, 41], xor(&vec![127, 93, 111], &vec![35, 68, 70]).unwrap());
    }

    #[test]
    fn xor_repeat_test() {
        assert_eq!(vec![92, 25, 41, 126, 43], xor_repeat(&vec![127, 93, 111, 93, 111],
                                                &vec![35, 68, 70]));
    }

    #[test]
    fn xor_char_test() {
        assert_eq!(vec![from_u32(57).unwrap(), from_u32(27).unwrap(), from_u32(41).unwrap()], xor_char(&vec![from_u32(127).unwrap(), from_u32(93).unwrap(), from_u32(111).unwrap()], 70).unwrap());
    }
}