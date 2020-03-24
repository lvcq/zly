use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use crate::zqueue::Queue;
use std::sync::{Arc, Mutex};

pub mod models;
pub mod schema;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub struct PgPool {
    workers: Vec<Arc<Mutex<DBWorker>>>,
    free: Arc<Mutex<Queue<usize>>>,
}

pub struct DBWorker {
    index: usize,
    free: Arc<Mutex<Queue<usize>>>,
   pub connection: PgConnection,
}

impl PgPool {
    pub fn new() -> Self {
        PgPool {
            workers: Vec::new(),
            free: Arc::new(Mutex::new(Queue::new())),
        }
    }

    pub fn get_free_worker(&mut self) -> Arc<Mutex<DBWorker>> {
        let mut free = self.free.lock().unwrap();
        if free.is_empty() {
            let index = self.workers.len();
            let dw = DBWorker::new(self.free.clone(), index);
            self.workers.push(Arc::new(Mutex::new(dw)));
            free.push(index);
        }
        let f_index = free.pop().unwrap();
        let f_w = self.workers.get(f_index).unwrap().clone();
        return f_w;
    }
}

impl DBWorker {
    fn new(free: Arc<Mutex<Queue<usize>>>, index: usize) -> Self {
        DBWorker {
            index,
            free,
            connection: establish_connection(),
        }
    }

    pub fn free(& self){
        self.free.lock().unwrap().push(self.index);
    }
}