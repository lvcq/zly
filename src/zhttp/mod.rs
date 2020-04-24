pub mod response_code;
pub mod zly_multipart;

use serde::Serialize;
use actix_web::HttpResponse;
use actix_web::http::header::CONTENT_TYPE;
use actix_session::Session;
pub use response_code::ResponseCode;
pub use zly_multipart::{FormDataValue, MultiFile};

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

/// 判断用户是否登录，已登录用户返回用户ID
pub fn user_auth(session: Session) -> Result<String, HttpResponse> {
    match session.get::<String>("userId") {
        Ok(Some(u_id)) => { Ok(u_id) }
        _ => {
            Err(response_json::<bool>(ResponseCode::Code10004.as_code(),
                                      Some(ResponseCode::Code10004.as_str().to_string()),
                                      false)
            )
        }
    }
}