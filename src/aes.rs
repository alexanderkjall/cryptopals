use crate::Error;
use std::arch::x86_64::{__m128i, _mm_loadu_si128, _mm_xor_si128, _mm_aesdec_si128, _mm_aesdeclast_si128, _mm_storeu_si128, _mm_aeskeygenassist_si128, _mm_shuffle_epi32, _mm_slli_si128, _mm_aesimc_si128, _mm_setzero_si128, _mm_aesenc_si128, _mm_aesenclast_si128, _mm_set_epi8};

pub enum AES_METHOD {
    ECB,
    CBC,
}

/// copied here from sse.rs so that we can run this in stable rust
#[inline]
#[allow(non_snake_case)]
const fn _MM_SHUFFLE(z: u32, y: u32, x: u32, w: u32) -> i32 {
    ((z << 6) | (y << 4) | (x << 2) | w) as i32
}

unsafe fn aes_128_key_expansion(key: __m128i, keygened: __m128i) -> __m128i {
    let keygened = _mm_shuffle_epi32(keygened, _MM_SHUFFLE(3,3,3,3));
    let key = _mm_xor_si128(key, _mm_slli_si128(key, 4));
    let key = _mm_xor_si128(key, _mm_slli_si128(key, 4));
    let key = _mm_xor_si128(key, _mm_slli_si128(key, 4));

    _mm_xor_si128(key, keygened)
}

unsafe fn aes128_load_key(enc_key: &[u8; 16], key_schedule: &mut [__m128i; 20]) {
    key_schedule[0] = _mm_loadu_si128(enc_key.as_ptr() as *const __m128i);
    key_schedule[1]  = aes_128_key_expansion(key_schedule[0], _mm_aeskeygenassist_si128(key_schedule[0], 0x01));
    key_schedule[2]  = aes_128_key_expansion(key_schedule[1], _mm_aeskeygenassist_si128(key_schedule[1], 0x02));
    key_schedule[3]  = aes_128_key_expansion(key_schedule[2], _mm_aeskeygenassist_si128(key_schedule[2], 0x04));
    key_schedule[4]  = aes_128_key_expansion(key_schedule[3], _mm_aeskeygenassist_si128(key_schedule[3], 0x08));
    key_schedule[5]  = aes_128_key_expansion(key_schedule[4], _mm_aeskeygenassist_si128(key_schedule[4], 0x10));
    key_schedule[6]  = aes_128_key_expansion(key_schedule[5], _mm_aeskeygenassist_si128(key_schedule[5], 0x20));
    key_schedule[7]  = aes_128_key_expansion(key_schedule[6], _mm_aeskeygenassist_si128(key_schedule[6], 0x40));
    key_schedule[8]  = aes_128_key_expansion(key_schedule[7], _mm_aeskeygenassist_si128(key_schedule[7], 0x80));
    key_schedule[9]  = aes_128_key_expansion(key_schedule[8], _mm_aeskeygenassist_si128(key_schedule[8], 0x1B));
    key_schedule[10] = aes_128_key_expansion(key_schedule[9], _mm_aeskeygenassist_si128(key_schedule[9], 0x36));

    key_schedule[11] = _mm_aesimc_si128(key_schedule[9]);
    key_schedule[12] = _mm_aesimc_si128(key_schedule[8]);
    key_schedule[13] = _mm_aesimc_si128(key_schedule[7]);
    key_schedule[14] = _mm_aesimc_si128(key_schedule[6]);
    key_schedule[15] = _mm_aesimc_si128(key_schedule[5]);
    key_schedule[16] = _mm_aesimc_si128(key_schedule[4]);
    key_schedule[17] = _mm_aesimc_si128(key_schedule[3]);
    key_schedule[18] = _mm_aesimc_si128(key_schedule[2]);
    key_schedule[19] = _mm_aesimc_si128(key_schedule[1]);
}

pub fn encrypt_aes_ebc(input: &[u8], key: &[u8; 16]) -> Result<Vec<u8>, Error> {
    let mut input = input.to_vec();
    add_padding(&mut input, 16)?;

    let mut cipher_text:Vec<u8> = vec![0u8; input.len()];
    unsafe {
        let mut key_schedule: [__m128i; 20] = [_mm_setzero_si128(); 20];

        aes128_load_key(key, &mut key_schedule);

        for i in 0..(input.len() / 16) {
            let mut m = _mm_loadu_si128((input.as_ptr() as *const __m128i).add(i));

            m = _mm_xor_si128(m, key_schedule[0]);
            m = _mm_aesenc_si128(m, key_schedule[1]);
            m = _mm_aesenc_si128(m, key_schedule[2]);
            m = _mm_aesenc_si128(m, key_schedule[3]);
            m = _mm_aesenc_si128(m, key_schedule[4]);
            m = _mm_aesenc_si128(m, key_schedule[5]);
            m = _mm_aesenc_si128(m, key_schedule[6]);
            m = _mm_aesenc_si128(m, key_schedule[7]);
            m = _mm_aesenc_si128(m, key_schedule[8]);
            m = _mm_aesenc_si128(m, key_schedule[9]);
            m = _mm_aesenclast_si128(m, key_schedule[10]);

            _mm_storeu_si128((cipher_text.as_ptr() as *mut __m128i).add(i), m);
        }
    }

    Ok(cipher_text)
}

pub fn decrypt_aes_ecb(input: &[u8], key: &[u8; 16]) -> Result<Vec<u8>, Error> {
    if input.len() % 16 != 0 {
        return Err(Error::Generic("block size isn't 16"))
    }

    let mut plain_text:Vec<u8> = vec![0u8; input.len()];
    unsafe {
        let mut key_schedule: [__m128i; 20] = [_mm_setzero_si128(); 20];

        aes128_load_key(key, &mut key_schedule);

        for i in 0..(input.len() / 16) {
            let mut m = _mm_loadu_si128((input.as_ptr() as *const __m128i).add(i));

            m = _mm_xor_si128(m, key_schedule[10]);
            m = _mm_aesdec_si128(m, key_schedule[11]);
            m = _mm_aesdec_si128(m, key_schedule[12]);
            m = _mm_aesdec_si128(m, key_schedule[13]);
            m = _mm_aesdec_si128(m, key_schedule[14]);
            m = _mm_aesdec_si128(m, key_schedule[15]);
            m = _mm_aesdec_si128(m, key_schedule[16]);
            m = _mm_aesdec_si128(m, key_schedule[17]);
            m = _mm_aesdec_si128(m, key_schedule[18]);
            m = _mm_aesdec_si128(m, key_schedule[19]);
            m = _mm_aesdeclast_si128(m, key_schedule[0]);

            _mm_storeu_si128((plain_text.as_ptr() as *mut __m128i).add(i), m);
        }
    }

    remove_padding(&mut plain_text)?;

    Ok(plain_text)
}

pub fn decrypt_aes_cbc(input: &[u8], key: &[u8; 16], iv: &[u8; 16]) -> Result<Vec<u8>, Error> {
    if input.len() % 16 != 0 {
        return Err(Error::Generic("block size isn't 16"))
    }

    let mut plain_text:Vec<u8> = vec![0u8; input.len()];
    unsafe {
        let iv: __m128i = _mm_set_epi8(iv[0] as i8, iv[1] as i8, iv[2] as i8, iv[3] as i8, iv[4] as i8, iv[5] as i8, iv[6] as i8, iv[7] as i8, iv[8] as i8, iv[9] as i8, iv[10] as i8, iv[11] as i8, iv[12] as i8, iv[13] as i8, iv[14] as i8, iv[15] as i8);
        let mut key_schedule: [__m128i; 20] = [_mm_setzero_si128(); 20];

        aes128_load_key(key, &mut key_schedule);

        for i in 0..(input.len() / 16) {
            let mut m = _mm_loadu_si128((input.as_ptr() as *const __m128i).add(i));

            m = _mm_xor_si128(m, key_schedule[10]);
            m = _mm_aesdec_si128(m, key_schedule[11]);
            m = _mm_aesdec_si128(m, key_schedule[12]);
            m = _mm_aesdec_si128(m, key_schedule[13]);
            m = _mm_aesdec_si128(m, key_schedule[14]);
            m = _mm_aesdec_si128(m, key_schedule[15]);
            m = _mm_aesdec_si128(m, key_schedule[16]);
            m = _mm_aesdec_si128(m, key_schedule[17]);
            m = _mm_aesdec_si128(m, key_schedule[18]);
            m = _mm_aesdec_si128(m, key_schedule[19]);
            m = _mm_aesdeclast_si128(m, key_schedule[0]);

            match i {
                0 => m = _mm_xor_si128(m, iv),
                _ => m = _mm_xor_si128(m, *(input.as_ptr() as * const __m128i).add(i - 1)),
            }

            _mm_storeu_si128((plain_text.as_ptr() as *mut __m128i).add(i), m);
        }
    }

    remove_padding(&mut plain_text)?;

    Ok(plain_text)
}

pub fn encrypt_aes_cbc(input: &[u8], key: &[u8; 16], iv: &[u8; 16]) -> Result<Vec<u8>, Error> {
    let mut input = input.to_vec();
    add_padding(&mut input, 16)?;

    let mut cipher_text:Vec<u8> = vec![0u8; input.len()];
    for (i, b) in iv.iter().enumerate() {
        cipher_text[i] = *b;
    }
    unsafe {
        let mut key_schedule: [__m128i; 20] = [_mm_setzero_si128(); 20];

        aes128_load_key(key, &mut key_schedule);

        for i in 0..(input.len() / 16) {
            let mut m = _mm_loadu_si128((input.as_ptr() as *const __m128i).add(i));

            m = _mm_xor_si128(m, *(cipher_text.as_ptr() as *const __m128i).add(match i { 0..=1 => 0, _ => i - 1}));

            m = _mm_xor_si128(m, key_schedule[0]);
            m = _mm_aesenc_si128(m, key_schedule[1]);
            m = _mm_aesenc_si128(m, key_schedule[2]);
            m = _mm_aesenc_si128(m, key_schedule[3]);
            m = _mm_aesenc_si128(m, key_schedule[4]);
            m = _mm_aesenc_si128(m, key_schedule[5]);
            m = _mm_aesenc_si128(m, key_schedule[6]);
            m = _mm_aesenc_si128(m, key_schedule[7]);
            m = _mm_aesenc_si128(m, key_schedule[8]);
            m = _mm_aesenc_si128(m, key_schedule[9]);
            m = _mm_aesenclast_si128(m, key_schedule[10]);

            _mm_storeu_si128((cipher_text.as_ptr() as *mut __m128i).add(i), m);
        }
    }

    Ok(cipher_text)
}

pub fn add_padding(plain_text: &mut Vec<u8>, block_size: usize) -> Result<(), Error> {
    if block_size == 0 {
        return Err(Error::Generic("block size must be > 0"));
    }

    let nr_of_bytes_to_extend = match plain_text.len() % block_size {
        0 => block_size,
        i => block_size - i
    };
    plain_text.extend(&vec![nr_of_bytes_to_extend as u8; nr_of_bytes_to_extend]);

    Ok(())
}

fn remove_padding(plain_text: &mut Vec<u8>) -> Result<(), Error> {
    if plain_text.is_empty() {
        return Err(Error::Generic("empty buffer, not padded"));
    }
    if plain_text.len() % 16 != 0 {
        return Err(Error::Generic("buffer not padded to 16"));
    }

    if plain_text[plain_text.len() - 1] < 17 && plain_text[plain_text.len() - 1] != 0 {
        let pad_byte = plain_text[plain_text.len() - 1];

        if (plain_text.len() as i32) - (pad_byte as i32) < 0 {
            return Err(Error::Generic("pad byte wrong"));
        }

        for i in 1..(pad_byte + 1) {
            if plain_text[plain_text.len() - i as usize] != pad_byte {
                return Err(Error::Generic("pad byte wrong"));
            }
        }

        plain_text.resize(plain_text.len() - pad_byte as usize, 0u8);
    } else {
        return Err(Error::Generic("pad byte wrong"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::aes::{remove_padding, add_padding, encrypt_aes_cbc, decrypt_aes_cbc};
    use crate::Error;

    #[test]
    fn cbc_loop() {
        let key = [b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'o', b'p', b'q'];
        let clean = "Quisque eget odio ac lectus vestibulum faucibus eget.";

        let enc = encrypt_aes_cbc(clean.as_bytes(), &key, &[0; 16]).unwrap();
        let result = decrypt_aes_cbc(&enc, &key, &[0; 16]).unwrap();

        let result = String::from_utf8(result).unwrap();

        assert_eq!(clean, result);
    }

    #[test]
    fn remove_padding_test_valid() {
        for length in 0..17 {
            let a = vec![length as u8; length];

            let mut a_padded = a.clone();
            a_padded.append(&mut vec![16 - (length % 16) as u8; 16 - (length % 16)]);

            remove_padding(&mut a_padded).unwrap();

            assert_eq!(a, a_padded);
        }
    }

    #[test]
    fn add_and_remove_padding_test_valid() {
        for length in 0..17 {
            let a = vec![length as u8; length];

            let mut a_padded = a.clone();
            add_padding(&mut a_padded, 16).unwrap();

            remove_padding(&mut a_padded).unwrap();

            assert_eq!(a, a_padded);
        }
    }

    #[test]
    fn remove_padding_test_empty_buf() {
        let mut a = vec![];

        let result = remove_padding(&mut a);

        let result = result.err().unwrap();
        assert!(matches!(result, Error::Generic("empty buffer, not padded")));
    }

    #[test]
    fn remove_padding_test_wrong_length() {
        let mut a = vec![3; 5];

        let result = remove_padding(&mut a);

        let result = result.err().unwrap();
        assert!(matches!(result, Error::Generic("buffer not padded to 16")));
    }

    #[test]
    fn remove_padding_test_not_padded() {
        let mut a = vec![20; 16];

        let result = remove_padding(&mut a);

        let result = result.err().unwrap();
        assert!(matches!(result, Error::Generic("pad byte wrong")));
    }

    #[test]
    fn remove_padding_test_not_padded_zero() {
        let mut a = vec![0; 16];

        let result = remove_padding(&mut a);

        let result = result.err().unwrap();
        assert!(matches!(result, Error::Generic("pad byte wrong")));
    }

    #[test]
    fn remove_padding_test_not_padded_matching_last_byte() {
        let mut a = vec![3; 16];
        a[15] = 5;

        let result = remove_padding(&mut a);

        let result = result.err().unwrap();
        assert!(matches!(result, Error::Generic("pad byte wrong")));
    }
}