use crate::xor::{guess_xor_byte_str, find_xored_english_string, xor_repeat, guess_xor_byte_with_space_vec};
use crate::hex::{parse_hex, to_hex};
use crate::xor::xor;
use crate::base64::{to_base64, from_base64};
use crate::file::{file_to_vec, file_to_buf};
use crate::string::{hamming_distance, trim_and_join};
use std::str::from_utf8;
use std::cmp::min;
use crate::aes::{decrypt_aes_ecb, add_padding, decrypt_aes_cbc};

mod aes;
mod hex;
mod base64;
mod xor;
mod file;
mod string;

#[derive(Debug)]
pub enum Error {
    Generic(&'static str),
    GenericStr(String),
    Utf8Error(std::str::Utf8Error),
    IoError(std::io::Error),
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Error {
        Error::Utf8Error(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IoError(err)
    }
}

fn number_of_duplicate_blocks(input: &[u8], key_size: usize) -> Result<usize, Error> {
    let mut parts = vec![];
    for i in 0..(input.len() / (key_size * 2)) {
        parts.push(&input[(i * key_size)..((i + 1) * key_size)]);
    }
    let mut adds = 0;
    for i in 0..parts.len() {
        for j in (i + 1)..parts.len() {
            if parts[i] == parts[j] {
                adds += 1;
            }
        }
    }
    Ok(adds)
}

fn score_edit_distance(input: &[u8], key_size: usize) -> Result<f64, Error> {
    let mut edit_delta = 0;
    let mut parts = vec![];
    for i in 0..(input.len() / (key_size * 2)) {
        parts.push(&input[(i * key_size)..((i + 1) * key_size)]);
    }
    let mut adds = 0;
    for i in 0..parts.len() {
        for j in (i + 1)..parts.len() {
            edit_delta += hamming_distance(parts[i], parts[j])?;
            adds += 1;
        }
    }

    Ok(edit_delta as f64 / (key_size * adds) as f64)
}

fn calc_edit_distance(input: &[u8]) -> Result<Vec<(f64, u8)>, Error> {
    let mut edit_deltas: Vec<(f64, u8)> = vec![];
    for key_size in 2..min(40, input.len() / 2) {
        edit_deltas.push((score_edit_distance(input, key_size)?, key_size as u8));
    }

    edit_deltas.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    Ok(edit_deltas)
}

fn score_edit_distance_16(input: &[Vec<u8>]) -> Result<usize, Error> {
    let mut edit_deltas: Vec<(usize, usize)> = vec![];
    for (i, buf) in input.iter().enumerate() {
        if !buf.is_empty() {
            edit_deltas.push((number_of_duplicate_blocks(&buf, 16)?, i));
        }
    }

    edit_deltas.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    Ok(edit_deltas[0].1)
}

fn decrypt_buf(input: &[u8]) -> Result<Vec<u8>, Error> {
    let edit_deltas = calc_edit_distance(input)?;

    //for a in 0..5 {
        let block_size = edit_deltas[0].1;

        let transposed = chop_and_transpose(input, block_size as usize);
        let xor_bytes = guess_xor_byte_with_space_vec(&transposed);
        //println!("{:?}", &xor_bytes);
        //println!("");
        //println!("{:?}", xor_repeat(input, &xor_bytes).iter().map(|&c| c as char).collect::<String>())
    //}
    //Err(Error::Generic("not done"))
    Ok(xor_repeat(input, &xor_bytes))
}

fn chop_and_transpose(input: &[u8], block_size: usize) -> Vec<Vec<u8>> {
    let mut out = Vec::with_capacity(block_size);
    let bytes_per_block = (input.len() + block_size - 1) / block_size;
    for i in 0..block_size {
        let mut v = Vec::with_capacity(bytes_per_block);
        for idx in 0..bytes_per_block {
            let pos = idx * block_size + i;
            if pos < input.len() {
                v.push(input[pos])
            } else {
                v.push(0);
            }
        }
        out.push(v);
    }
    out
}

fn solve_1_1() -> Result<(), Error> {
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let result = to_base64(&parse_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")?)?;

    assert_eq!(expected, result);

    println!("exp = {}\nres = {}", expected, result);

    Ok(())
}

fn solve_1_2() -> Result<(), Error> {
    let expected = "746865206b696420646f6e277420706c6179";

    let result = to_hex(&xor(&parse_hex("1c0111001f010100061a024b53535009181c")?, &parse_hex("686974207468652062756c6c277320657965")?)?);

    assert_eq!(expected, result);

    println!("exp = {}\nres = {}", expected, result);

    Ok(())
}

fn solve_1_3() -> Result<(), Error> {
    let expected = "Cooking MC's like a pound of bacon";

    let result = guess_xor_byte_str(&std::str::from_utf8(&parse_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")?)?);

    assert_eq!(expected, result);

    println!("exp = {}\nres = {}", expected, result);

    Ok(())
}

fn solve_1_4() -> Result<(), Error> {
    let expected = "Now that the party is jumping\n";
    let result = find_xored_english_string(
                   &file_to_vec("res/4.txt")?.iter()
                       .map(|s| String::from_utf8_lossy(&parse_hex(s).unwrap()).to_string()).collect::<Vec<String>>());

    assert_eq!(expected, result);

    println!("exp = {}\nres = {}", expected, result);

    Ok(())
}

fn solve_1_5() -> Result<(), Error> {
    let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    let result = to_hex(&xor_repeat(&"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal".bytes().collect::<Vec<u8>>(), &[b'I', b'C', b'E']));

    assert_eq!(expected, result);

    println!("exp = {}\nres = {}", expected, result);

    Ok(())
}

fn solve_1_6() -> Result<(), Error> {
    let expected = "I\'m back and I\'m ringin\' the bell \nA rockin\' on the mike while the fly girls yell \nIn ecstasy in the back of me \nWell that\'s my DJ Deshay cuttin\' all them Z\'s \nHittin\' hard and the girlies goin\' crazy \nVanilla\'s on the mike, man I\'m not lazy. \n\nI\'m lettin\' my drug kick in \nIt controls my mouth and I begin \nTo just let it flow, let my concepts go \nMy posse\'s to the side yellin\', Go Vanilla Go! \n\nSmooth \'cause that\'s the way I will be \nAnd if you don\'t give a damn, then \nWhy you starin\' at me \nSo get off \'cause I control the stage \nThere\'s no dissin\' allowed \nI\'m in my own phase \nThe girlies sa y they love me and that is ok \nAnd I can dance better than any kid n\' play \n\nStage 2 -- Yea the one ya\' wanna listen to \nIt\'s off my head so let the beat play through \nSo I can funk it up and make it sound good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I like my rhymes atrocious \nSupercalafragilisticexpialidocious \nI\'m an effect and that you can bet \nI can take a fly girl and make her wet. \n\nI\'m like Samson -- Samson to Delilah \nThere\'s no denyin\', You can try to hang \nBut you\'ll keep tryin\' to get my style \nOver and over, practice makes perfect \nBut not if you\'re a loafer. \n\nYou\'ll get nowhere, no place, no time, no girls \nSoon -- Oh my God, homebody, you probably eat \nSpaghetti with a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I\'m comin\' hard like a rhino \nIntoxicating so you stagger like a wino \nSo punks stop trying and girl stop cryin\' \nVanilla Ice is sellin\' and you people are buyin\' \n\'Cause why the freaks are jockin\' like Crazy Glue \nMovin\' and groovin\' trying to sing along \nAll through the ghetto groovin\' this here song \nNow you\'re amazed by the VIP posse. \n\nSteppin\' so hard like a German Nazi \nStartled by the bases hittin\' ground \nThere\'s no trippin\' on mine, I\'m just gettin\' down \nSparkamatic, I\'m hangin\' tight like a fanatic \nYou trapped me once and I thought that \nYou might have it \nSo step down and lend me your ear \n\'89 in my time! You, \'90 is my year. \n\nYou\'re weakenin\' fast, YO! and I can tell it \nYour body\'s gettin\' hot, so, so I can smell it \nSo don\'t be mad and don\'t be sad \n\'Cause the lyrics belong to ICE, You can call me Dad \nYou\'re pitchin\' a fit, so step back and endure \nLet the witch doctor, Ice, do the dance to cure \nSo come up close and don\'t be square \nYou wanna battle me -- Anytime, anywhere \n\nYou thought that I was weak, Boy, you\'re dead wrong \nSo come on, everybody and sing this song \n\nSay -- Play that funky music Say, go white boy, go white boy go \nplay that funky music Go white boy, go white boy, go \nLay down and boogie and play that funky music till you die. \n\nPlay that funky music Come on, Come on, let me hear \nPlay that funky music white boy you say it, say it \nPlay that funky music A little louder now \nPlay that funky music, white boy Come on, Come on, Come on \nPlay that funky music \n";
    let f = file_to_buf("res/6.txt")?;
    let b64 = from_utf8(&f)?;
    let interim = decrypt_buf(&from_base64(&trim_and_join(b64))?)?;
    let result = from_utf8(&interim)?;

    assert_eq!(expected, result);

    println!("exp = {}\nres = {}", expected, result);

    Ok(())
}

fn solve_1_7() -> Result<(), Error> {
    let expected = "I\'m back and I\'m ringin\' the bell \nA rockin\' on the mike while the fly girls yell \nIn ecstasy in the back of me \nWell that\'s my DJ Deshay cuttin\' all them Z\'s \nHittin\' hard and the girlies goin\' crazy \nVanilla\'s on the mike, man I\'m not lazy. \n\nI\'m lettin\' my drug kick in \nIt controls my mouth and I begin \nTo just let it flow, let my concepts go \nMy posse\'s to the side yellin\', Go Vanilla Go! \n\nSmooth \'cause that\'s the way I will be \nAnd if you don\'t give a damn, then \nWhy you starin\' at me \nSo get off \'cause I control the stage \nThere\'s no dissin\' allowed \nI\'m in my own phase \nThe girlies sa y they love me and that is ok \nAnd I can dance better than any kid n\' play \n\nStage 2 -- Yea the one ya\' wanna listen to \nIt\'s off my head so let the beat play through \nSo I can funk it up and make it sound good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I like my rhymes atrocious \nSupercalafragilisticexpialidocious \nI\'m an effect and that you can bet \nI can take a fly girl and make her wet. \n\nI\'m like Samson -- Samson to Delilah \nThere\'s no denyin\', You can try to hang \nBut you\'ll keep tryin\' to get my style \nOver and over, practice makes perfect \nBut not if you\'re a loafer. \n\nYou\'ll get nowhere, no place, no time, no girls \nSoon -- Oh my God, homebody, you probably eat \nSpaghetti with a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I\'m comin\' hard like a rhino \nIntoxicating so you stagger like a wino \nSo punks stop trying and girl stop cryin\' \nVanilla Ice is sellin\' and you people are buyin\' \n\'Cause why the freaks are jockin\' like Crazy Glue \nMovin\' and groovin\' trying to sing along \nAll through the ghetto groovin\' this here song \nNow you\'re amazed by the VIP posse. \n\nSteppin\' so hard like a German Nazi \nStartled by the bases hittin\' ground \nThere\'s no trippin\' on mine, I\'m just gettin\' down \nSparkamatic, I\'m hangin\' tight like a fanatic \nYou trapped me once and I thought that \nYou might have it \nSo step down and lend me your ear \n\'89 in my time! You, \'90 is my year. \n\nYou\'re weakenin\' fast, YO! and I can tell it \nYour body\'s gettin\' hot, so, so I can smell it \nSo don\'t be mad and don\'t be sad \n\'Cause the lyrics belong to ICE, You can call me Dad \nYou\'re pitchin\' a fit, so step back and endure \nLet the witch doctor, Ice, do the dance to cure \nSo come up close and don\'t be square \nYou wanna battle me -- Anytime, anywhere \n\nYou thought that I was weak, Boy, you\'re dead wrong \nSo come on, everybody and sing this song \n\nSay -- Play that funky music Say, go white boy, go white boy go \nplay that funky music Go white boy, go white boy, go \nLay down and boogie and play that funky music till you die. \n\nPlay that funky music Come on, Come on, let me hear \nPlay that funky music white boy you say it, say it \nPlay that funky music A little louder now \nPlay that funky music, white boy Come on, Come on, Come on \nPlay that funky music \n";
    let key: [u8; 16] = [b'Y', b'E', b'L', b'L', b'O', b'W', b' ', b'S', b'U', b'B', b'M', b'A', b'R', b'I', b'N', b'E'];

    let f = file_to_buf("res/7.txt")?;
    let b64 = from_utf8(&f)?;
    let interim = decrypt_aes_ecb(&from_base64(&trim_and_join(b64))?, &key)?;
    let result = from_utf8(&interim)?;

    assert_eq!(expected, result);

    println!("exp = {}\nres = {}", expected, result);

    Ok(())
}

fn solve_1_8() -> Result<(), Error> {
    let expected = 132;

    let f:Vec<Vec<u8>> = file_to_vec("res/8.txt")?.iter().map(|s| parse_hex(&s)).collect::<Result<Vec<Vec<u8>>, Error>>()?;

    let result = score_edit_distance_16(&f)?;

    assert_eq!(expected, result);

    println!("exp = {}\nres = {}", expected, result);

    Ok(())
}

fn solve_2_9() -> Result<(), Error> {
    let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 8, 8, 8, 8, 8, 8, 8];

    let mut result = vec![0, 1, 2, 3, 4, 5, 6, 7];
    add_padding(&mut result, 16)?;

    assert_eq!(expected, result);

    println!("exp = {:?}\nres = {:?}", expected, result);

    Ok(())
}

fn solve_2_10() -> Result<(), Error> {
    let expected = "I\'m back and I\'m ringin\' the bell \nA rockin\' on the mike while the fly girls yell \nIn ecstasy in the back of me \nWell that\'s my DJ Deshay cuttin\' all them Z\'s \nHittin\' hard and the girlies goin\' crazy \nVanilla\'s on the mike, man I\'m not lazy. \n\nI\'m lettin\' my drug kick in \nIt controls my mouth and I begin \nTo just let it flow, let my concepts go \nMy posse\'s to the side yellin\', Go Vanilla Go! \n\nSmooth \'cause that\'s the way I will be \nAnd if you don\'t give a damn, then \nWhy you starin\' at me \nSo get off \'cause I control the stage \nThere\'s no dissin\' allowed \nI\'m in my own phase \nThe girlies sa y they love me and that is ok \nAnd I can dance better than any kid n\' play \n\nStage 2 -- Yea the one ya\' wanna listen to \nIt\'s off my head so let the beat play through \nSo I can funk it up and make it sound good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I like my rhymes atrocious \nSupercalafragilisticexpialidocious \nI\'m an effect and that you can bet \nI can take a fly girl and make her wet. \n\nI\'m like Samson -- Samson to Delilah \nThere\'s no denyin\', You can try to hang \nBut you\'ll keep tryin\' to get my style \nOver and over, practice makes perfect \nBut not if you\'re a loafer. \n\nYou\'ll get nowhere, no place, no time, no girls \nSoon -- Oh my God, homebody, you probably eat \nSpaghetti with a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I\'m comin\' hard like a rhino \nIntoxicating so you stagger like a wino \nSo punks stop trying and girl stop cryin\' \nVanilla Ice is sellin\' and you people are buyin\' \n\'Cause why the freaks are jockin\' like Crazy Glue \nMovin\' and groovin\' trying to sing along \nAll through the ghetto groovin\' this here song \nNow you\'re amazed by the VIP posse. \n\nSteppin\' so hard like a German Nazi \nStartled by the bases hittin\' ground \nThere\'s no trippin\' on mine, I\'m just gettin\' down \nSparkamatic, I\'m hangin\' tight like a fanatic \nYou trapped me once and I thought that \nYou might have it \nSo step down and lend me your ear \n\'89 in my time! You, \'90 is my year. \n\nYou\'re weakenin\' fast, YO! and I can tell it \nYour body\'s gettin\' hot, so, so I can smell it \nSo don\'t be mad and don\'t be sad \n\'Cause the lyrics belong to ICE, You can call me Dad \nYou\'re pitchin\' a fit, so step back and endure \nLet the witch doctor, Ice, do the dance to cure \nSo come up close and don\'t be square \nYou wanna battle me -- Anytime, anywhere \n\nYou thought that I was weak, Boy, you\'re dead wrong \nSo come on, everybody and sing this song \n\nSay -- Play that funky music Say, go white boy, go white boy go \nplay that funky music Go white boy, go white boy, go \nLay down and boogie and play that funky music till you die. \n\nPlay that funky music Come on, Come on, let me hear \nPlay that funky music white boy you say it, say it \nPlay that funky music A little louder now \nPlay that funky music, white boy Come on, Come on, Come on \nPlay that funky music \n";
    let key: [u8; 16] = [b'Y', b'E', b'L', b'L', b'O', b'W', b' ', b'S', b'U', b'B', b'M', b'A', b'R', b'I', b'N', b'E'];

    let f = file_to_buf("res/10.txt")?;
    let b64 = from_utf8(&f)?;
    let interim = decrypt_aes_cbc(&from_base64(&trim_and_join(b64))?, &key, &[0; 16])?;
    let result = from_utf8(&interim)?;

    assert_eq!(expected, result);

    println!("exp = {}\nres = {}", expected, result);

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("wrong number of arguments");
        return;
    }

    match args[1].as_str() {
        "1.1" => {
            solve_1_1().unwrap();
        },
        "1.2" => {
            solve_1_2().unwrap();
        },
        "1.3" => {
            solve_1_3().unwrap();
        },
        "1.4" => {
            solve_1_4().unwrap();
        },
        "1.5" => {
            solve_1_5().unwrap();
        },
        "1.6" => {
            solve_1_6().unwrap();
        },
        "1.7" => {
            solve_1_7().unwrap();
        },
        "1.8" => {
            solve_1_8().unwrap();
        },
        "2.9" => {
            solve_2_9().unwrap();
        },
        "2.10" => {
            solve_2_10().unwrap();
        },
        _ => {
            eprintln!("unknown argument")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hex::{parse_hex, to_hex};
    use crate::base64::{to_base64, from_base64};
    use crate::file::{file_to_vec, file_to_buf};
    use crate::xor::{xor, xor_repeat, guess_xor_byte_str, find_xored_english_string};
    use crate::{chop_and_transpose, decrypt_buf};
    use std::str::from_utf8;
    use crate::string::trim_and_join;
    use crate::aes::{decrypt_aes_ecb, add_padding, decrypt_aes_cbc};

    #[test]
    fn test_chop_and_transpose_aligned() {
        let input = vec![0, 1, 2, 3, 4, 5];
        let result = chop_and_transpose(&input, 3);

        assert_eq!(3, result.len());

        assert_eq!(2, result[0].len());
        assert_eq!(2, result[1].len());
        assert_eq!(2, result[2].len());

        assert_eq!(0, result[0][0]);
        assert_eq!(3, result[0][1]);

        assert_eq!(1, result[1][0]);
        assert_eq!(4, result[1][1]);

        assert_eq!(2, result[2][0]);
        assert_eq!(5, result[2][1]);
    }

    #[test]
    fn test_chop_and_transpose_with_padding() {
        let input = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let result = chop_and_transpose(&input, 3);

        assert_eq!(3, result.len());

        assert_eq!(3, result[0].len());
        assert_eq!(3, result[1].len());
        assert_eq!(3, result[2].len());

        assert_eq!(0, result[0][0]);
        assert_eq!(3, result[0][1]);
        assert_eq!(6, result[0][2]);

        assert_eq!(1, result[1][0]);
        assert_eq!(4, result[1][1]);
        assert_eq!(7, result[1][2]);

        assert_eq!(2, result[2][0]);
        assert_eq!(5, result[2][1]);
        assert_eq!(0, result[2][2]);
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
                   guess_xor_byte_str(&std::str::from_utf8(&parse_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap()).unwrap()));
    }

    #[test]
    fn crypto_pals_challenge4_complete() {
        assert_eq!("Now that the party is jumping\n",
                   &find_xored_english_string(
                       &file_to_vec("res/4.txt").unwrap().iter()
                           .map(|s| String::from_utf8_lossy(&parse_hex(s).unwrap()).to_string()).collect::<Vec<String>>()));
    }

    #[test]
    fn crypto_pals_challenge5_complete() {
        assert_eq!("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f",
            to_hex(&xor_repeat(&"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal".bytes().collect::<Vec<u8>>(), &vec![b'I', b'C', b'E'])));
    }

    #[test]
    fn crypto_pals_challenge6_complete() {
        let f = file_to_buf("res/6.txt").unwrap();
        let b64 = from_utf8(&f).unwrap();
        let interim = decrypt_buf(&from_base64(&trim_and_join(b64)).unwrap()).unwrap();
        let result = from_utf8(&interim).unwrap();

        assert_eq!("I\'m back and I\'m ringin\' the bell \nA rockin\' on the mike while the fly girls yell \nIn ecstasy in the back of me \nWell that\'s my DJ Deshay cuttin\' all them Z\'s \nHittin\' hard and the girlies goin\' crazy \nVanilla\'s on the mike, man I\'m not lazy. \n\nI\'m lettin\' my drug kick in \nIt controls my mouth and I begin \nTo just let it flow, let my concepts go \nMy posse\'s to the side yellin\', Go Vanilla Go! \n\nSmooth \'cause that\'s the way I will be \nAnd if you don\'t give a damn, then \nWhy you starin\' at me \nSo get off \'cause I control the stage \nThere\'s no dissin\' allowed \nI\'m in my own phase \nThe girlies sa y they love me and that is ok \nAnd I can dance better than any kid n\' play \n\nStage 2 -- Yea the one ya\' wanna listen to \nIt\'s off my head so let the beat play through \nSo I can funk it up and make it sound good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I like my rhymes atrocious \nSupercalafragilisticexpialidocious \nI\'m an effect and that you can bet \nI can take a fly girl and make her wet. \n\nI\'m like Samson -- Samson to Delilah \nThere\'s no denyin\', You can try to hang \nBut you\'ll keep tryin\' to get my style \nOver and over, practice makes perfect \nBut not if you\'re a loafer. \n\nYou\'ll get nowhere, no place, no time, no girls \nSoon -- Oh my God, homebody, you probably eat \nSpaghetti with a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I\'m comin\' hard like a rhino \nIntoxicating so you stagger like a wino \nSo punks stop trying and girl stop cryin\' \nVanilla Ice is sellin\' and you people are buyin\' \n\'Cause why the freaks are jockin\' like Crazy Glue \nMovin\' and groovin\' trying to sing along \nAll through the ghetto groovin\' this here song \nNow you\'re amazed by the VIP posse. \n\nSteppin\' so hard like a German Nazi \nStartled by the bases hittin\' ground \nThere\'s no trippin\' on mine, I\'m just gettin\' down \nSparkamatic, I\'m hangin\' tight like a fanatic \nYou trapped me once and I thought that \nYou might have it \nSo step down and lend me your ear \n\'89 in my time! You, \'90 is my year. \n\nYou\'re weakenin\' fast, YO! and I can tell it \nYour body\'s gettin\' hot, so, so I can smell it \nSo don\'t be mad and don\'t be sad \n\'Cause the lyrics belong to ICE, You can call me Dad \nYou\'re pitchin\' a fit, so step back and endure \nLet the witch doctor, Ice, do the dance to cure \nSo come up close and don\'t be square \nYou wanna battle me -- Anytime, anywhere \n\nYou thought that I was weak, Boy, you\'re dead wrong \nSo come on, everybody and sing this song \n\nSay -- Play that funky music Say, go white boy, go white boy go \nplay that funky music Go white boy, go white boy, go \nLay down and boogie and play that funky music till you die. \n\nPlay that funky music Come on, Come on, let me hear \nPlay that funky music white boy you say it, say it \nPlay that funky music A little louder now \nPlay that funky music, white boy Come on, Come on, Come on \nPlay that funky music \n",
            result);
    }

    #[test]
    fn crypto_pals_challenge7_complete() {
        let key: [u8; 16] = [b'Y', b'E', b'L', b'L', b'O', b'W', b' ', b'S', b'U', b'B', b'M', b'A', b'R', b'I', b'N', b'E'];
        let f = file_to_buf("res/7.txt").unwrap();
        let b64 = from_utf8(&f).unwrap();
        let interim = decrypt_aes_ecb(&from_base64(&trim_and_join(b64)).unwrap(), &key).unwrap();
        let result = from_utf8(&interim).unwrap();

        assert_eq!("I'm back and I'm ringin' the bell \nA rockin' on the mike while the fly girls yell \nIn ecstasy in the back of me \nWell that's my DJ Deshay cuttin' all them Z's \nHittin' hard and the girlies goin' crazy \nVanilla's on the mike, man I'm not lazy. \n\nI'm lettin' my drug kick in \nIt controls my mouth and I begin \nTo just let it flow, let my concepts go \nMy posse's to the side yellin', Go Vanilla Go! \n\nSmooth 'cause that's the way I will be \nAnd if you don't give a damn, then \nWhy you starin' at me \nSo get off 'cause I control the stage \nThere's no dissin' allowed \nI'm in my own phase \nThe girlies sa y they love me and that is ok \nAnd I can dance better than any kid n' play \n\nStage 2 -- Yea the one ya' wanna listen to \nIt's off my head so let the beat play through \nSo I can funk it up and make it sound good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I like my rhymes atrocious \nSupercalafragilisticexpialidocious \nI'm an effect and that you can bet \nI can take a fly girl and make her wet. \n\nI'm like Samson -- Samson to Delilah \nThere's no denyin', You can try to hang \nBut you'll keep tryin' to get my style \nOver and over, practice makes perfect \nBut not if you're a loafer. \n\nYou'll get nowhere, no place, no time, no girls \nSoon -- Oh my God, homebody, you probably eat \nSpaghetti with a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I'm comin' hard like a rhino \nIntoxicating so you stagger like a wino \nSo punks stop trying and girl stop cryin' \nVanilla Ice is sellin' and you people are buyin' \n'Cause why the freaks are jockin' like Crazy Glue \nMovin' and groovin' trying to sing along \nAll through the ghetto groovin' this here song \nNow you're amazed by the VIP posse. \n\nSteppin' so hard like a German Nazi \nStartled by the bases hittin' ground \nThere's no trippin' on mine, I'm just gettin' down \nSparkamatic, I'm hangin' tight like a fanatic \nYou trapped me once and I thought that \nYou might have it \nSo step down and lend me your ear \n'89 in my time! You, '90 is my year. \n\nYou're weakenin' fast, YO! and I can tell it \nYour body's gettin' hot, so, so I can smell it \nSo don't be mad and don't be sad \n'Cause the lyrics belong to ICE, You can call me Dad \nYou're pitchin' a fit, so step back and endure \nLet the witch doctor, Ice, do the dance to cure \nSo come up close and don't be square \nYou wanna battle me -- Anytime, anywhere \n\nYou thought that I was weak, Boy, you're dead wrong \nSo come on, everybody and sing this song \n\nSay -- Play that funky music Say, go white boy, go white boy go \nplay that funky music Go white boy, go white boy, go \nLay down and boogie and play that funky music till you die. \n\nPlay that funky music Come on, Come on, let me hear \nPlay that funky music white boy you say it, say it \nPlay that funky music A little louder now \nPlay that funky music, white boy Come on, Come on, Come on \nPlay that funky music \n",
            result);
    }

    #[test]
    fn crypto_pals_challenge9_complete() {
        let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 8, 8, 8, 8, 8, 8, 8];

        let mut result = vec![0, 1, 2, 3, 4, 5, 6, 7];
        add_padding(&mut result, 16).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn crypto_pals_challenge10_complete() {
        let key: [u8; 16] = [b'Y', b'E', b'L', b'L', b'O', b'W', b' ', b'S', b'U', b'B', b'M', b'A', b'R', b'I', b'N', b'E'];
        let f = file_to_buf("res/10.txt").unwrap();
        let b64 = from_utf8(&f).unwrap();
        let interim = decrypt_aes_cbc(&from_base64(&trim_and_join(b64)).unwrap(), &key, &[0; 16]).unwrap();
        let result = from_utf8(&interim).unwrap();

        assert_eq!("I'm back and I'm ringin' the bell \nA rockin' on the mike while the fly girls yell \nIn ecstasy in the back of me \nWell that's my DJ Deshay cuttin' all them Z's \nHittin' hard and the girlies goin' crazy \nVanilla's on the mike, man I'm not lazy. \n\nI'm lettin' my drug kick in \nIt controls my mouth and I begin \nTo just let it flow, let my concepts go \nMy posse's to the side yellin', Go Vanilla Go! \n\nSmooth 'cause that's the way I will be \nAnd if you don't give a damn, then \nWhy you starin' at me \nSo get off 'cause I control the stage \nThere's no dissin' allowed \nI'm in my own phase \nThe girlies sa y they love me and that is ok \nAnd I can dance better than any kid n' play \n\nStage 2 -- Yea the one ya' wanna listen to \nIt's off my head so let the beat play through \nSo I can funk it up and make it sound good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I like my rhymes atrocious \nSupercalafragilisticexpialidocious \nI'm an effect and that you can bet \nI can take a fly girl and make her wet. \n\nI'm like Samson -- Samson to Delilah \nThere's no denyin', You can try to hang \nBut you'll keep tryin' to get my style \nOver and over, practice makes perfect \nBut not if you're a loafer. \n\nYou'll get nowhere, no place, no time, no girls \nSoon -- Oh my God, homebody, you probably eat \nSpaghetti with a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I'm comin' hard like a rhino \nIntoxicating so you stagger like a wino \nSo punks stop trying and girl stop cryin' \nVanilla Ice is sellin' and you people are buyin' \n'Cause why the freaks are jockin' like Crazy Glue \nMovin' and groovin' trying to sing along \nAll through the ghetto groovin' this here song \nNow you're amazed by the VIP posse. \n\nSteppin' so hard like a German Nazi \nStartled by the bases hittin' ground \nThere's no trippin' on mine, I'm just gettin' down \nSparkamatic, I'm hangin' tight like a fanatic \nYou trapped me once and I thought that \nYou might have it \nSo step down and lend me your ear \n'89 in my time! You, '90 is my year. \n\nYou're weakenin' fast, YO! and I can tell it \nYour body's gettin' hot, so, so I can smell it \nSo don't be mad and don't be sad \n'Cause the lyrics belong to ICE, You can call me Dad \nYou're pitchin' a fit, so step back and endure \nLet the witch doctor, Ice, do the dance to cure \nSo come up close and don't be square \nYou wanna battle me -- Anytime, anywhere \n\nYou thought that I was weak, Boy, you're dead wrong \nSo come on, everybody and sing this song \n\nSay -- Play that funky music Say, go white boy, go white boy go \nplay that funky music Go white boy, go white boy, go \nLay down and boogie and play that funky music till you die. \n\nPlay that funky music Come on, Come on, let me hear \nPlay that funky music white boy you say it, say it \nPlay that funky music A little louder now \nPlay that funky music, white boy Come on, Come on, Come on \nPlay that funky music \n",
                   result);
    }
}