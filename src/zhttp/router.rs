use super::middleware::{HttpPhase, ZRequest};
use crate::zqueue::Queue;
use http::{Method, Response,StatusCode};
use hyper::Body;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type HandlerFn= Box<dyn Fn(&mut ZRequest,&mut Response<Body>,&mut HttpPhase) + Send + 'static>;

pub struct RouterItem {
    pub method: Option<Vec<Method>>,
    pub handler: HandlerFn,
}
pub struct Router {
    workers: Vec<Arc<Mutex<RouterWorker>>>,
    free_worker: Arc<Mutex<Queue<u16>>>,
    create_router_map:Box<dyn Fn()->HashMap<String,RouterItem> +Send + 'static>
}

pub struct RouterWorker {
    id: u16,
    free_worker: Arc<Mutex<Queue<u16>>>,
    router_map: HashMap<String,RouterItem>
}

impl Router {
    pub fn new(f:Box<dyn Fn()->HashMap<String,RouterItem> + Send + 'static>) -> Router {
        Router {
            workers: Vec::new(),
            free_worker: Arc::new(Mutex::new(Queue::new())),
            create_router_map:f
        }
    }

    pub fn get_free_worker(&mut self) -> Arc<Mutex<RouterWorker>> {
        let mut free_worker = self.free_worker.lock().unwrap();
        if free_worker.is_empty() {
            let index = self.workers.len();
            let worker = RouterWorker::new(index as u16,
                                           self.free_worker.clone(),
                                           (self.create_router_map)());
            self.workers.push(Arc::new(Mutex::new(worker)));
            free_worker.push(index as u16);
        }
        let f_index = free_worker.pop().unwrap();
        return self.workers.get(f_index as usize).unwrap().clone();
    }
}

impl RouterWorker {
    pub fn new(id: u16, free_worker: Arc<Mutex<Queue<u16>>>,router_map:HashMap<String,RouterItem>) -> Self {
        RouterWorker { id, free_worker,router_map }
    }
    /// #### 路由处理完成后释放worker
    ///
    pub fn free_worker(&self) {
        self.free_worker.lock().unwrap().push(self.id);
    }
    
    pub fn handler_request(&self,zreq:&mut ZRequest,response:&mut Response<Body>, hp:&mut HttpPhase){
        let url = zreq.req.uri().path();
        let item: &RouterItem;
        let default_key= String::from("default-router");
        if self.router_map.contains_key(url){
            item= self.router_map.get(url).unwrap();
        } else if self.router_map.contains_key(&default_key){
            item = self.router_map.get(&default_key).unwrap();
        } else {
            *response.status_mut()=StatusCode::NOT_FOUND;
            *response.body_mut()=Body::from("not found");
            return ();
        }
        (item.handler)(zreq,response,hp);
    }
}
