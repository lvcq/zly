use chrono::{NaiveDateTime, Utc};
use crypto::digest::Digest;
use crypto::sha3::Sha3;

pub mod short_id;
pub mod hash;

pub fn current_naive_datetime() -> NaiveDateTime {
    Utc::now().naive_utc()
}

pub fn crypto_password(password: &str) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(password);
    hasher.result_str()
}

pub fn crypto_password_with_username_timestamp(
    p_str: &str,
    username: &str,
    timestamp: u64,
    res_str: &str) -> bool {
    let cry_str = format!("{}-{}-{}",p_str,username,timestamp);
    let mut hasher =Sha3::sha3_256();
    hasher.input_str(&cry_str);
    res_str.eq(&hasher.result_str())
}


#[cfg(test)]
mod test {
    use crate::yutils::{crypto_password_with_username_timestamp};

    #[test]
    fn test_hmac_crypto() {
        let pass_str = "abc";
        let username = "test";
        let timestamp: u64 = 112;
        let res_str = "fb0c7d7c60de64c9751f9762c81adb3105e8caaef1dce41989b7e394d64291b2";
        assert!(crypto_password_with_username_timestamp(pass_str,username,timestamp,res_str))
    }
}