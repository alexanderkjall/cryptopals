use crate::Error;
use std::arch::x86_64::{__m128i, _mm_loadu_si128, _mm_xor_si128, _mm_aesdec_si128, _mm_aesdeclast_si128, _mm_storeu_si128, _mm_aeskeygenassist_si128, _mm_shuffle_epi32, _mm_slli_si128, _mm_aesimc_si128, _mm_setzero_si128};

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
    return _mm_xor_si128(key, keygened);
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

pub fn decrypt_aes_ecb(input: &[u8], key: &[u8; 16]) -> Result<Vec<u8>, Error> {
    if input.len() % 16 != 0 {
        return Err(Error::Generic("block size isn't 16"))
    }

    let mut plain_text:Vec<u8> = vec![0u8; input.len()];
    unsafe {
        let mut key_schedule: [__m128i; 20] = [_mm_setzero_si128(); 20];

        aes128_load_key(key, &mut key_schedule);

        for i in 0..(input.len() / 16) {
            let mut m = _mm_loadu_si128((input.as_ptr() as *const __m128i).offset(i as isize));

            m = _mm_xor_si128(m, key_schedule[10 + 0]);
            m = _mm_aesdec_si128(m, key_schedule[10 + 1]);
            m = _mm_aesdec_si128(m, key_schedule[10 + 2]);
            m = _mm_aesdec_si128(m, key_schedule[10 + 3]);
            m = _mm_aesdec_si128(m, key_schedule[10 + 4]);
            m = _mm_aesdec_si128(m, key_schedule[10 + 5]);
            m = _mm_aesdec_si128(m, key_schedule[10 + 6]);
            m = _mm_aesdec_si128(m, key_schedule[10 + 7]);
            m = _mm_aesdec_si128(m, key_schedule[10 + 8]);
            m = _mm_aesdec_si128(m, key_schedule[10 + 9]);
            m = _mm_aesdeclast_si128(m, key_schedule[0]);

            _mm_storeu_si128((plain_text.as_ptr() as *mut __m128i).offset(i as isize), m);
        }
    }

    find_and_remove_padding(&mut plain_text);

    Ok(plain_text)
}

fn find_and_remove_padding(plain_text: &mut Vec<u8>) {
    if plain_text.len() == 0 {
        return;
    }

    if plain_text[plain_text.len() - 1] < 17 {
        let pad_byte = plain_text[plain_text.len() - 1];

        if (plain_text.len() as i32) - (pad_byte as i32) < 0 {
            return;
        }

        for i in 1..(pad_byte + 1) {
            if plain_text[plain_text.len() - i as usize] != pad_byte {
                return;
            }
        }

        plain_text.resize(plain_text.len() - pad_byte as usize, 0u8);
    }
}