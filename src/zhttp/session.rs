use crate::zredis::{RedisConfig, RedisPool};
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::digest::Digest;
use crypto::sha2::Sha512;
use crypto::{aes, blockmodes, buffer, symmetriccipher};
use uuid::Uuid;

pub struct Session {
    redis_conn_pool: RedisPool,
    prefix: String,
    secret: String,
}

#[derive(Clone)]
pub struct SessionConfig {
    pub prefix: Option<String>,
    pub secret: Option<String>,
    pub redis_config: RedisConfig,
}

impl Session {
    pub fn new(config: SessionConfig) -> Self {
        let prefix = match config.prefix {
            Some(p_str) => p_str,
            None => String::from("zly"),
        };
        let secret = match config.secret {
            Some(sec) => sec,
            None => String::from("010893"),
        };
        let mut hasher = Sha512::new();
        hasher.input_str(&secret);
        let hex = hasher.result_str();
        let pool = RedisPool::new(config.redis_config);
        Session {
            prefix,
            secret: hex,
            redis_conn_pool: pool,
        }
    }

    pub fn store_session(&mut self, text: &str) -> Result<String, String> {
        let e_arr = self.encrypt_str(text)?;
        let key_uuid = self.create_session_key();
        let e_str = serde_json::to_string(&e_arr).unwrap();
        let key = format!("{}:{}", self.prefix, key_uuid);
        self.redis_conn_pool.set::<&str>(&key, &e_str)?;
        Ok(key_uuid)
    }

    fn encrypt_str(&self, text: &str) -> Result<Vec<u8>, String> {
        let data_bytes = text.as_bytes();
        let key_bytes = self.secret.as_bytes();
        let res_vec: Vec<u8>;
        match self.aes256_cbc_encrypt(data_bytes, &key_bytes[0..16], &key_bytes[0..16]) {
            Ok(res) => res_vec = res,
            Err(_) => return Err(String::from("session 加密失败")),
        }
        Ok(res_vec)
    }

    fn decrypt_str(&self, ciphertext: &[u8]) -> Result<String, String> {
        let key_bytes = self.secret.as_bytes();
        let res_vec: Vec<u8>;
        match self.aes256_cbc_decrypt(ciphertext, &key_bytes[0..16], &key_bytes[0..16]) {
            Ok(res) => Ok(String::from_utf8_lossy(&res[..]).to_string()),
            Err(_) => return Err(String::from("session 解密失败")),
        }
    }

    pub fn get_session_by_key(&mut self, key: &str) -> Option<String> {
        let redis_key = format!("{}:{}", self.prefix, key);
        match self.redis_conn_pool.get::<String>(&redis_key) {
            Some(res) => {
                let vec_str: Vec<&str> = res
                    .trim_matches(|p| p == '[' || p == ']')
                    .split(',')
                    .collect();
                let vec_u8: Vec<u8> = vec_str
                    .into_iter()
                    .map(|i| i.parse::<u8>().unwrap())
                    .collect();
                match self.decrypt_str(&vec_u8[..]) {
                    Ok(st) => Some(st),
                    Err(e) => {
                        println!("{}", e);
                        None
                    }
                }
            }
            None => return None,
        }
    }

    fn aes256_cbc_encrypt(
        &self,
        data: &[u8],
        key: &[u8],
        iv: &[u8],
    ) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
        let mut encryptor =
            aes::cbc_encryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);
        let mut result_vec: Vec<u8> = Vec::new();
        let mut read_buffer = buffer::RefReadBuffer::new(data);
        let mut r_buffer: [u8; 8192] = [0; 8192];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut r_buffer);
        loop {
            let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;
            result_vec.extend(
                write_buffer
                    .take_read_buffer()
                    .take_remaining()
                    .iter()
                    .map(|&i| i),
            );
            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => {}
            }
        }
        Ok(result_vec)
    }

    fn aes256_cbc_decrypt(
        &self,
        data: &[u8],
        key: &[u8],
        iv: &[u8],
    ) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
        let mut decryptor =
            aes::cbc_decryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);
        let mut final_result: Vec<u8> = Vec::new();
        let mut read_buffer = buffer::RefReadBuffer::new(data);
        let mut r_buffer: [u8; 8192] = [0; 8192];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut r_buffer);
        loop {
            let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
            final_result.extend(
                write_buffer
                    .take_read_buffer()
                    .take_remaining()
                    .iter()
                    .map(|&i| i),
            );
            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => {}
            }
        }
        Ok(final_result)
    }

    fn create_session_key(&self) -> String {
        let uid = Uuid::new_v4();
        uid.to_string()
    }
}
