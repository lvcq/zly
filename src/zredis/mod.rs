use super::zqueue::Queue;
use redis;
use redis::Commands;
use redis::Connection;

#[derive(Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub database: u8,
    pub auth: String,
}

impl Into<String> for RedisConfig{
    fn into(self) -> String {
        return format!(
            "redis://:{}@{}:{}/{}",
            self.auth, self.host, self.port, self.database
        );
    }
}

/// ### redis 连接池实现
///

pub struct RedisPool {
    conn_str: String,
    free_conn_ids: Queue<usize>,
    conns: Vec<Box<Connection>>,
}

impl RedisPool {
    pub fn new(config: RedisConfig) -> RedisPool {
        let conn_str = format!(
            "redis://:{}@{}:{}/{}",
            config.auth, config.host, config.port, config.database
        );
        let mut rpool = RedisPool {
            conn_str,
            free_conn_ids: Queue::new(),
            conns: Vec::new(),
        };
        match rpool.create_new_conn() {
            Ok(_) => println!("连接redis成功."),
            Err(_) => println!("连接redis失败"),
        };
        return rpool;
    }

    fn create_new_conn(&mut self) -> redis::RedisResult<isize> {
        let client = redis::Client::open(self.conn_str.clone())?;
        let conn: Connection = client.get_connection()?;
        let index: usize = self.conns.len();
        self.free_conn_ids.push(index);
        self.conns.push(Box::new(conn));
        println!("新建redis连接成功");
        Ok(1)
    }

    pub fn set<T>(&mut self, key: &str, value: T) -> Result<(), String>
    where
        T: redis::ToRedisArgs,
    {
        let conn_index: usize = self.get_free_conn()?;
        println!("redis-set 使用连接:{},key:{} ", &conn_index, &key);
        let conn = match self.conns.get_mut(conn_index) {
            Some(free_conn) => free_conn,
            None => return Err(String::from("redis set fail")),
        };
        match conn.set::<&str, T, ()>(key, value) {
            Ok(_) => {
                self.free_conn_ids.push(conn_index);
                return Ok(());
            }
            Err(_) => {
                self.free_conn_ids.push(conn_index);
                return Err(String::from("redis set fail"));
            }
        }
    }

    pub fn get<T>(&mut self, key: &str) -> Option<T>
    where
        T: redis::FromRedisValue,
    {
        let conn_index: usize = match self.get_free_conn() {
            Ok(index) => index,
            Err(_) => return None,
        };
        println!("redis-get 使用连接:{},key:{} ", &conn_index, &key);
        let conn = match self.conns.get_mut(conn_index) {
            Some(free_conn) => free_conn,
            None => return None,
        };
        match conn.get(key) {
            Ok(value) => {
                self.free_conn_ids.push(conn_index);
                return Some(value);
            }
            Err(_) => {
                println!("获取失败");
                self.free_conn_ids.push(conn_index);
                return None;
            }
        }
    }

    fn get_free_conn(&mut self) -> Result<usize, String> {
        if self.free_conn_ids.is_empty() {
            match self.create_new_conn() {
                Ok(_) => {}
                Err(_) => {
                    println!("新建redis连接失败");
                    return Err(String::from("redis error"));
                }
            }
        }
        Ok(self.free_conn_ids.pop().unwrap())
    }
}
