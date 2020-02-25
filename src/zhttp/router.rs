use std::sync::{Arc,Mutex};
use crate::zqueue::Queue;
use std::collections::HashMap;
use http::{Method,Response};
use hyper::Body;
use super::middleware::{ZRequest,HttpPhase};


pub struct RouterItem{
   pub method:Option<Vec<Method>>,
   pub handler: Box<dyn Fn(&mut ZRequest,&mut Response<Body>,&mut HttpPhase) +Send + 'static>
}
pub struct Router{
    workers:Vec< Arc<Mutex<RouterWorker>>>,
    free_worker: Arc<Mutex<Queue<u16>>>,
    router_map:HashMap<String,RouterItem>,
}

pub struct RouterWorker{
   id:u16,
   free_worker:Arc<Mutex<Queue<u16>>>
}

impl Router{
    pub fn new()->Router{
        Router{
            workers: Vec::new(),
            free_worker: Arc::new(Mutex::new(Queue::new())),
            router_map: HashMap::new()
        }
    }
    
    pub fn add_router(mut self,path:&str,router_item:RouterItem)->Self{
        let p_str= String::from(path).to_lowercase();
        if !self.router_map.contains_key(&p_str) && !p_str.eq(""){
            self.router_map.insert(p_str,router_item);
        }
        self
    }

    fn wait_free_worker_unlock(& self){
        loop{
            if !self.free_worker.is_poisoned(){
                break;
            }
        }
    }

    pub fn get_free_worker(&mut self)->Arc<Mutex<RouterWorker>>{
       // self.wait_free_worker_unlock();
        let mut free_worker=self.free_worker.lock().unwrap();
        if free_worker.is_empty(){
            let index =self.workers.len();
            let worker = RouterWorker::new(index as u16, self.free_worker.clone());
            self.workers.push(Arc::new(Mutex::new(worker)));
            free_worker.push(index as u16);
        }
        let mut free_worker= self.free_worker.lock().unwrap();
        let f_index = free_worker.pop().unwrap();
        return self.workers.get(f_index as usize).unwrap().clone();    
    }
}

impl RouterWorker{
    pub fn new(id:u16,free_worker:Arc<Mutex<Queue<u16>>>)->Self{
        RouterWorker{
            id,
            free_worker
        }
    }
    pub fn free_worker(&self){
        self.free_worker.lock().unwrap().push(self.id);
    }
}

