use std::collections::HashMap;
use crate::Error;

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

pub fn calc_char_percentages(chars: &[char]) -> HashMap<char, f64> {
    let mut map = HashMap::new();

    for c in chars.iter() {
        map.insert(*c, *map.get(c).unwrap_or(&0.0) + 1.0);
    }

    for (_, val) in map.iter_mut() {
        *val = (*val / chars.len() as f64) * 100.0;
    }
    map
}

pub fn calc_percentage_diff(percentages: HashMap<char, f64>) -> f64 {
    let mut accumulated_diff = 0.0;

    for key in percentages.keys() {
        accumulated_diff += f64::abs(percentages.get(key).unwrap_or(&0.0) - english_char_percentage(*key));
    }

    accumulated_diff
}

pub fn hamming_distance(a: &[u8], b: &[u8]) -> Result<usize, Error> {
    if a.len() != b.len() {
        return Err(Error::Generic("a and b is not of the same length"))
    }

    let mut ones: usize = 0;

    a.iter().zip(b.iter()).for_each(|(a, b)| ones += (a ^ b).count_ones() as usize);

    Ok(ones)
}

pub fn trim_and_join(a: &str) -> String {
    a.replace(&[' ', '\n'][..], "")
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::string::{calc_char_percentages, hamming_distance};

    #[test]
    fn calc_char_percentages_test() {
        let mut expected = HashMap::new();
        expected.insert('a', 50.0);
        expected.insert('b', 50.0);
        assert_eq!(expected,
                   calc_char_percentages(&vec!['a', 'a', 'b', 'b']));
    }

    #[test]
    fn calc_char_percentages_test_different_case() {
        let mut expected = HashMap::new();
        expected.insert('a', 25.0);
        expected.insert('A', 25.0);
        expected.insert('b', 25.0);
        expected.insert('B', 25.0);
        assert_eq!(expected,
                   calc_char_percentages(&vec!['a', 'A', 'b', 'B']));
    }

    #[test]
    fn hamming_distance_test() {
        assert_eq!(37, hamming_distance("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()).unwrap())
    }

    #[test]
    fn hamming_distance_test_2() {
        assert_eq!(6, hamming_distance("jake".as_bytes(), "fire".as_bytes()).unwrap())
    }
}