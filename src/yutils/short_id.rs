use chrono::Utc;
use rand::prelude::*;
use std::iter::FromIterator;

static SHORT_ID_SYMBOL: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
    'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
static BENCHMARK_TIME: i64 = 726883200000;
static SHORT_ID_BASE: i64 = 62;

pub fn generate_short_id(len: usize) -> String {
    let timestamp = Utc::now().timestamp_millis();
    let diff = timestamp - BENCHMARK_TIME;
    let mut s_vec = decimal_to_base(diff);
    if len > s_vec.len() {
        let salt_len = len - s_vec.len();
        let mut salt_vec = create_salt(salt_len);
        s_vec.append(&mut salt_vec);
    }

    String::from_iter(s_vec)
}

fn decimal_to_base(input: i64) -> Vec<char> {
    let mut decimal = input;
    let mut vec_base: Vec<char> = Vec::new();
    while decimal >= 1 {
        let index = decimal - SHORT_ID_BASE * (decimal / SHORT_ID_BASE);
        let t_char = SHORT_ID_SYMBOL[index as usize];
        vec_base.insert(0, t_char);
        decimal = decimal / SHORT_ID_BASE
    }
    vec_base
}

fn create_salt(len: usize) -> Vec<char> {
    let mut rng = rand::thread_rng();
    let ran: f64 = rng.gen::<f64>();
    let factor = SHORT_ID_BASE.pow(len as u32);
    let salt_num = (ran * (factor as f64)) as i64;
    return decimal_to_base(salt_num);
}


#[cfg(test)]
mod short_id_test {
    use super::generate_short_id;

    #[test]
    fn test_short_id() {
        let len: usize = 12;
        let id = generate_short_id(len);
        println!("short id:{}",id.clone());
        assert_eq!(id, String::from(""))
    }
}