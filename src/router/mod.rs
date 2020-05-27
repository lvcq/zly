use crate::lfile;
use crate::lstorage;
use crate::lsystem;
use crate::lsystem::{LoginInfo, RootInfo};
use crate::luser;
use crate::luser::ShowUserInfo;
use crate::zhttp::response_code::ResponseCode;
use crate::zhttp::{response_json, user_auth, HttpResult};
use crate::zpostgres::PgPool;
use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
pub mod file_upload;
pub mod img;

pub use file_upload::{
    check_file_exist_in_folder, create_ref_with_exist_file_and_storage, file_upload_handler,
};

pub use img::{image_parse_handler, image_parse_with_size_or_format, origin_image_handler};

#[derive(Deserialize)]
pub struct NewStorage {
    pub storage_name: String,
}

#[derive(Deserialize)]
pub struct QueryStorageFile {
    pub storage_id: String,
}

pub async fn validate_logon(
    session: Session,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::error::Error> {
    let mut code: usize = 20000;
    let u_info: Option<ShowUserInfo>;
    let mut msg: String = "".to_string();
    let db_worker = db.get_free_worker().unwrap();
    if let Some(user_id) = session.get::<String>("userId").expect("get user id error") {
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

pub async fn set_root_info(
    db: web::Data<PgPool>,
    info: web::Json<RootInfo>,
) -> Result<HttpResponse, actix_web::Error> {
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
    info: web::Json<LoginInfo>,
) -> Result<HttpResponse, actix_web::Error> {
    let db_worker = db.get_free_worker().unwrap();
    let mut msg: Option<String> = None;
    let mut code: usize = 20000;
    let user_info: Option<ShowUserInfo>;
    match luser::validate_user_password(
        &info.username,
        &info.password,
        info.timestamp,
        &db_worker.connection,
    ) {
        Ok(u_info) => {
            let user_id = u_info.user_id.clone();
            session.set("userId", user_id).expect("set session fail");
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
    db.free(db_worker.index);
    Ok(response_json::<Option<ShowUserInfo>>(code, msg, user_info))
}

pub async fn add_new_storage_handler(
    db: web::Data<PgPool>,
    session: Session,
    storage_info: web::Json<NewStorage>,
) -> Result<HttpResponse, actix_web::Error> {
    let db_worker = db.get_free_worker().unwrap();
    let mut msg: Option<String> = None;
    let mut code: usize = 20000;
    let mut add_success = false;
    let user_id = match session.get::<String>("userId") {
        Ok(u_id) => u_id.unwrap(),
        Err(_) => {
            db.free(db_worker.index);
            return Ok(response_json::<bool>(
                ResponseCode::Code10004.as_code(),
                Some(ResponseCode::Code10004.as_str().to_string()),
                false,
            ));
        }
    };
    match lstorage::add_new_storage(
        user_id,
        storage_info.storage_name.clone(),
        &db_worker.connection,
    ) {
        Ok(_) => {
            add_success = true;
        }
        Err(rsc) => {
            code = rsc.as_code();
            msg = Some(rsc.as_str().to_string());
        }
    }
    db.free(db_worker.index);
    return Ok(response_json::<bool>(code, msg, add_success));
}

pub async fn get_user_storage(
    db: web::Data<PgPool>,
    session: Session,
) -> Result<HttpResponse, actix_web::Error> {
    let db_worker = db.get_free_worker().unwrap();
    let mut msg: Option<String> = None;
    let mut code: usize = 20000;
    let mut storage_list: Option<Vec<lstorage::StorageInfo>> = None;
    let user_id = match session.get::<String>("userId") {
        Ok(u_id) => u_id.unwrap(),
        Err(_) => {
            db.free(db_worker.index);
            return Ok(response_json::<bool>(
                ResponseCode::Code10004.as_code(),
                Some(ResponseCode::Code10004.as_str().to_string()),
                false,
            ));
        }
    };

    match lstorage::get_storage_list_by_user_id(user_id, &db_worker.connection) {
        Ok(s_l) => {
            storage_list = Some(s_l);
        }
        Err(rsc) => {
            code = rsc.as_code();
            msg = Some(rsc.as_str().to_string());
        }
    }
    db.free(db_worker.index);
    return Ok(response_json::<Option<Vec<lstorage::StorageInfo>>>(
        code,
        msg,
        storage_list,
    ));
}

/// 处理请求空间文件
pub async fn query_files(
    query: web::Query<QueryStorageFile>,
    db: web::Data<PgPool>,
    session: Session,
) -> Result<HttpResponse, actix_web::Error> {
    let _user_id = match user_auth(session) {
        Ok(u_id) => u_id,
        Err(rsp) => {
            return Ok(rsp);
        }
    };
    let db_worker = db.get_free_worker().unwrap();
    let mut code: usize = 20000;
    let mut msg: Option<String> = None;
    let mut files: Option<Vec<lfile::FileItem>> = None;
    match lfile::query_file_list_by_storage_id(&query.storage_id, &db_worker.connection).await {
        Ok(file_vec) => {
            files = Some(file_vec);
        }
        Err(rsc) => {
            code = rsc.as_code();
            msg = Some(rsc.as_str().to_string());
        }
    }
    db.free(db_worker.index);

    Ok(response_json::<Option<Vec<lfile::FileItem>>>(
        code, msg, files,
    ))
}
