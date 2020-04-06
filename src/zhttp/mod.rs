use std::net::SocketAddr;


pub mod session;
pub mod response_code;

use serde::Serialize;
use actix_web::HttpResponse;
use actix_web::http::header::CONTENT_TYPE;


#[derive(Serialize)]
pub struct HttpResult<T>
    where T: Serialize
{
    pub success: bool,
    pub code: usize,
    pub message: Option<String>,
    pub data: T,
}


pub fn response_json<T>(code: usize, message: Option<String>, data: T) -> HttpResponse
    where T: Serialize
{
    let h_res: HttpResult<T> = HttpResult {
        success: true,
        code,
        message,
        data,
    };
     HttpResponse::Ok()
        .set_header(CONTENT_TYPE, "application/json; charset=utf-8")
        .body(serde_json::to_string(&h_res).unwrap())
}