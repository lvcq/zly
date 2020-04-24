use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use crate::zqueue::Queue;
use std::cell::RefCell;
use std::rc::Rc;

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
    workers: RefCell<Vec<Rc<DBWorker>>>,
    free: RefCell<Queue<usize>>,
}

pub struct DBWorker {
    pub index: usize,
    pub connection: PgConnection,
}

impl<'a> PgPool {
    pub fn new() -> Self {
        PgPool {
            workers: RefCell::new(Vec::new()),
            free: RefCell::new(Queue::new()),
        }
    }

    pub fn get_free_worker(&self) -> Option<Rc<DBWorker>> {
        if self.free.borrow().is_empty() {
            let index = self.workers.borrow().len();
            let dw = DBWorker::new(index);
            self.workers.borrow_mut().push(Rc::new(dw));
            self.free.borrow_mut().push(index);
        }
        let f_index = self.free.borrow_mut().pop().unwrap();
        self.workers.borrow().get(f_index).map(|x| x.clone())
    }

    pub fn free(&self, index: usize) {
        self.free.borrow_mut().push(index);
    }
}

impl DBWorker {
    fn new(index: usize) -> Self {
        DBWorker {
            index,
            connection: establish_connection(),
        }
    }
}