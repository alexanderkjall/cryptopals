use crate::Error;
use std::fs::File;
use std::io::Read;

pub fn file_to_buf(filename: &str) -> Result<Vec<u8>, Error> {
    let mut file = File::open(filename)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}

pub fn file_to_vec(filename: &str) -> Result<Vec<String>, Error> {
    let mut file = File::open(filename)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    data.split(|b| *b == b'\n').map(|buf|
        match std::str::from_utf8(buf) {
            Ok(s) => Ok(s.to_owned()),
            Err(e) => Err(Error::Utf8Error(e))
        }).collect()
}