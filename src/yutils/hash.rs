use crypto::digest::Digest;
use crypto::sha3::Sha3;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufReader, Read};

const DEFAULT_BLOCK_SIZE: u64 = 65536;

pub fn calc_file_hash(file_path: &str) -> Result<String, std::io::Error> {
    let file = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(file_path)?;
    let mut hasher = Sha3::sha3_256();
    let mut buffered = BufReader::new(file);
    let size = fs::metadata(file_path).unwrap().len();
    if size <= DEFAULT_BLOCK_SIZE {
        let mut data: Vec<u8> = Vec::with_capacity(size as usize);
        buffered.read_to_end(data.as_mut())?;
        hasher.input(&data);
    } else {
        let mut position = 0;
        while size - position > 0 {
            let data_size: usize = if size - position >= DEFAULT_BLOCK_SIZE {
                DEFAULT_BLOCK_SIZE as usize
            } else {
                (size - position) as usize
            };
            let mut data: Vec<u8> = vec![0; data_size];
            buffered.read_exact(data.as_mut())?;
            position += data.len() as u64;
            hasher.input(&data);
        }
    }
    Ok(hasher.result_str())
}
