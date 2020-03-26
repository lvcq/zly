use hyper::{Body, Request};
use std::sync::{Arc, Mutex};
use crate::zpostgres::DBWorker;
use zformdata::FormValue;
use http::{Uri, HeaderMap, Method};

pub struct ZRequest {
    pub session: ZSession,
    pub db_worker: Arc<Mutex<DBWorker>>,
    formdata: FormValue,
    uri: Uri,
    headers: HeaderMap,
    method: Method,
}

impl ZRequest {
    pub fn new(req: Request<Body>, db_worker: Arc<Mutex<DBWorker>>) -> Self {
        let uri = req.uri().clone();
        let headers = req.headers().clone();
        let method = req.method().clone();
        let fv = zformdata::read_formdata(req);
        println!("fv::{:?}",&fv);
        ZRequest {
            session: ZSession::new(),
            db_worker,
            formdata: fv,
            uri,
            headers,
            method,
        }
    }

    pub fn set_session_is_new(&mut self, is_n: bool) {
        self.session.set_is_new(is_n);
    }

    pub fn uri(&self) -> &Uri {
        return &self.uri;
    }

    pub fn formdata(&self) -> &FormValue {
        return &self.formdata;
    }

    pub fn headers(&self) -> &HeaderMap {
        return &self.headers;
    }

    pub fn method(&self) -> &Method {
        return &self.method;
    }
}

pub struct ZSession {
    pub value: Option<String>,
    pub key: Option<String>,
    pub is_new: bool,
}

impl ZSession {
    pub fn new() -> Self {
        ZSession {
            value: None,
            key: None,
            is_new: false,
        }
    }

    pub fn set_is_new(&mut self, is_n: bool) {
        self.is_new = is_n;
    }

    pub fn set_value(&mut self, value: &str) {
        self.value = Some(String::from(value));
    }

    pub fn set_value_none(&mut self) {
        self.value = None;
    }

    pub fn get_value(&self) -> Option<String> {
        match &self.value {
            Some(str_v) => Some(str_v.clone()),
            None => None,
        }
    }

    pub fn set_key(&mut self, key: &str) {
        self.key = Some(String::from(key));
    }

    pub fn set_key_none(&mut self) {
        self.key = None;
    }

    pub fn get_key(&self) -> Option<String> {
        match &self.key {
            Some(str_k) => Some(str_k.clone()),
            None => None
        }
    }
}
