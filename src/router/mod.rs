use actix_web::{HttpResponse, web};
use actix_session::Session;
use crate::zpostgres::PgPool;
use crate::lsystem;
use crate::luser;
use crate::lsystem::{RootInfo, LoginInfo};
use crate::zhttp::{HttpResult, response_json};
use std::borrow::Borrow;
use crate::luser::ShowUserInfo;
use crate::zhttp::response_code::ResponseCode;
use crate::zpostgres::models::user_info::UserInfo;
use http::header::CONTENT_TYPE;

pub async fn validate_logon(session: Session, db: web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    let mut code: usize = 20000;
    let u_info: Option<ShowUserInfo>;
    let mut msg: String = "".to_string();
    let db_worker = db.get_free_worker().unwrap();
    if let Some(user_id) = session.get::<String>("userId")? {
        u_info = match luser::validate_user(user_id, &db_worker.connection) {
            Ok(sui) => Some(sui),
            Err(err) => {
                code = err.as_code();
                msg = err.as_str().to_string();
                None
            }
        }
    } else {
        msg = ResponseCode::Code10002.as_str().to_string();
        code = ResponseCode::Code10002.as_code();
        u_info = None
    }
    if u_info.is_none() {}
    let res: HttpResult<Option<ShowUserInfo>> = HttpResult {
        success: true,
        code,
        message: Some(msg),
        data: u_info,
    };
    db.free(db_worker.index);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn validate_init(db: web::Data<PgPool>) -> HttpResponse {
    let db_worker = db.get_free_worker().unwrap();
    let is_init = lsystem::is_init(&db_worker.connection);
    let res: HttpResult<bool> = HttpResult {
        success: true,
        code: 20000,
        message: None,
        data: is_init,
    };
    db.free(db_worker.index);
    HttpResponse::Ok().body(serde_json::to_string(&res).unwrap())
}

pub async fn set_root_info(db: web::Data<PgPool>, info: web::Json<RootInfo>) -> Result<HttpResponse, actix_web::Error> {
    let db_worker = db.get_free_worker().unwrap();
    let mut msg: Option<String> = None;
    let mut code: usize = 20000;
    let is_success = match lsystem::set_root_info(info, &db_worker.connection) {
        Ok(_) => true,
        Err(err) => {
            msg = Some(err.as_str().to_string());
            code = err.as_code();
            false
        }
    };
    let res: HttpResult<bool> = HttpResult {
        success: true,
        code,
        message: msg,
        data: is_success,
    };
    db.free(db_worker.index);
    Ok(HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()))
}

pub async fn user_login(
    db: web::Data<PgPool>,
    session: Session,
    info: web::Json<LoginInfo>) -> Result<HttpResponse, actix_web::Error> {
    let db_worker = db.get_free_worker().unwrap();
    let mut msg: Option<String> = None;
    let mut code: usize = 20000;
    let mut user_info: Option<ShowUserInfo> = None;
    match luser::validate_user_password(
        &info.username,
        &info.password,
        info.timestamp,
        &db_worker.connection) {
        Ok(u_info) => {
            let user_id = u_info.user_id.clone();
            session.set("userId", user_id)?;
            user_info = Some(ShowUserInfo {
                username: u_info.user_name.clone(),
                email: u_info.email.clone(),
                last_login_time: None,
            });
        }
        Err(r_code) => {
            user_info = None;
            code = r_code.as_code();
            msg = Some(r_code.as_str().to_string());
        }
    }
    Ok(response_json::<Option<ShowUserInfo>>(code, msg, user_info))
}