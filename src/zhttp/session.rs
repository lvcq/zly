use crate::zredis::{RedisPool,RedisConfig};
use crypto::{symmetriccipher,buffer,aes,blockmodes};
use crypto::buffer::{ReadBuffer,WriteBuffer,BufferResult};
use uuid::Uuid;

pub struct Session{
    redis_conn_pool: RedisPool,
    prefix:String,
    secret:String,
}

pub struct SessionConfig{
   pub prefix:Option<String>,
   pub  secret:Option<String>,
   pub  redis_config:RedisConfig,
}

impl Session{
    pub fn new(config:SessionConfig)->Self{
        let prefix = match config.prefix{
            Some(p_str)=>p_str,
            None=>String::from("zly")
        };
        let secret = match config.secret {
            Some(sec)=>sec,
            None=>String::from("010893")
        };
        let pool = RedisPool::new(config.redis_config);
        Session{
            prefix,
            secret,
            redis_conn_pool:pool
        }
    }

    pub async fn store_session(&mut self,text:&str)->Result<String,String>{
       let e_str=self.encrypt_str(text)?;
       let key = self.create_session_key();
       self.redis_conn_pool.set::<&[u8]>(&key,&e_str[..]).await?;
       Ok(key)
    }

    fn encrypt_str(&self,text:&str)->Result<Vec<u8>,String>{
        let data_bytes= text.as_bytes();
        let key_bytes = self.secret.as_bytes();
        let res_vec:Vec<u8>;
        match self.aes256_cbc_encrypt(data_bytes, key_bytes, key_bytes){
            Ok(res)=>res_vec = res,
            Err(_)=>{
                return Err(String::from("session 加密失败"))
            }
        }
        Ok(res_vec)

    }

    fn aes256_cbc_encrypt(&self,data:&[u8],key:&[u8],iv:&[u8])->Result<Vec<u8>,symmetriccipher::SymmetricCipherError>{
        let mut encryptor = aes::cbc_encryptor(
            aes::KeySize::KeySize256,
            key,
            iv,
            blockmodes::PkcsPadding);
        let mut result_vec:Vec<u8> = Vec::new();
        let mut read_buffer = buffer::RefReadBuffer::new(data);
        let mut r_buffer:[u8;4096]=[0;4096];
        let mut write_buffer= buffer::RefWriteBuffer::new(&mut r_buffer);
        loop {
            let result=encryptor.encrypt(&mut read_buffer,&mut write_buffer,true)?;
            result_vec.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
            match result {
                BufferResult::BufferUnderflow=>break,
                BufferResult::BufferOverflow=>{},
            }
        }
        Ok(result_vec)
    }
    
    fn create_session_key(&self)->String{
        let uid = Uuid::new_v4();
        format!("{}:{}",self.prefix,uid)
    }

}

