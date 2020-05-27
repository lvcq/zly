pub mod response_code;
pub mod zly_multipart;

use actix_session::Session;
use actix_web::http::header::CONTENT_TYPE;
use actix_web::HttpResponse;
pub use response_code::ResponseCode;
use serde::Serialize;
pub use zly_multipart::{FormDataValue, MultiFile};

#[derive(Serialize)]
pub struct HttpResult<T>
where
    T: Serialize,
{
    pub success: bool,
    pub code: usize,
    pub message: Option<String>,
    pub data: T,
}

pub fn response_json<T>(code: usize, message: Option<String>, data: T) -> HttpResponse
where
    T: Serialize,
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

pub fn generate_response_with_response_code(
    rsc: ResponseCode,
    would_throw_error: bool,
) -> HttpResponse {
    let code: usize = rsc.as_code();
    let msg: Option<String> = Some(rsc.as_str().to_string());
    return if would_throw_error {
        HttpResponse::BadRequest()
        .set_header(CONTENT_TYPE, "text/plain; charset=utf-8").body(rsc.as_str().to_string())
    } else {
        response_json::<Option<String>>(code, msg, None)
    };
}

/// 判断用户是否登录，已登录用户返回用户ID
pub fn user_auth(session: Session) -> Result<String, HttpResponse> {
    match session.get::<String>("userId") {
        Ok(Some(u_id)) => Ok(u_id),
        _ => Err(response_json::<bool>(
            ResponseCode::Code10004.as_code(),
            Some(ResponseCode::Code10004.as_str().to_string()),
            false,
        )),
    }
}
