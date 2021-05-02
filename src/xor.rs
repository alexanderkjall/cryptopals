use crate::Error;

pub fn xor(a: &[u8], b: &[u8]) -> Result<Vec<u8>, Error> {
    if a.len() != b.len() {
        return Err(Error::Generic("a and b is not equal in length"))
    }

    Ok(a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect())
}
