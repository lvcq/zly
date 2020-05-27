use crate::lfile;
use crate::zconfig::AppConfig;
use crate::zhttp::zly_multipart::transfer_multipart;
use crate::zhttp::{response_json, user_auth, ResponseCode};
use crate::zpostgres::PgPool;
use actix_multipart::Multipart;
use actix_session::Session;
use actix_web::web;
use actix_web::HttpResponse;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct HashInfo {
    pub hash: String,
}

#[derive(Deserialize)]
pub struct FileExistInFolderInfo {
    pub hash: String,
    pub storage_id: String,
    pub file_name: String,
}

pub async fn file_upload_handler(
    db: web::Data<PgPool>,
    app_config: web::Data<AppConfig>,
    session: Session,
    payload: Multipart,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = match user_auth(session) {
        Ok(u_id) => u_id,
        Err(rsp) => {
            return Ok(rsp);
        }
    };
    let fdv = transfer_multipart(payload).await?;
    let mut code: usize = 20000;
    let mut msg: Option<String> = None;
    let mut f_id: Option<String> = None;
    let storage_id = fdv.fields.get("storage_id");
    let multi_file = fdv.files.get("zly_file");
    let file_hash = fdv.fields.get("file_hash");
    if storage_id.is_some() && multi_file.is_some() {
        let storage_id = storage_id.unwrap().clone();
        let multi_file = multi_file.unwrap();
        let db_worker = db.get_free_worker().unwrap();
        match lfile::store_file(
            multi_file,
            &app_config.file_storage_path,
            user_id,
            storage_id,
            &file_hash.unwrap(),
            &db_worker.connection,
        )
        .await
        {
            Ok(file_id) => {
                f_id = Some(file_id);
            }
            Err(rsc) => {
                code = rsc.as_code();
                msg = Some(rsc.as_str().to_string());
            }
        }
        db.free(db_worker.index);
    } else {
        code = ResponseCode::Code10005.as_code();
        msg = Some(ResponseCode::Code10005.as_str().to_string());
    };
    Ok(response_json::<Option<String>>(code, msg, f_id))
}

pub async fn check_file_exist_in_folder(
    info: web::Json<HashInfo>,
    app_config: web::Data<AppConfig>,
    session: Session,
) -> Result<HttpResponse, actix_web::Error> {
    match user_auth(session) {
        Ok(_) => {}
        Err(rsp) => {
            return Ok(rsp);
        }
    };

    let mut code: usize = 20000;
    let mut msg: Option<String> = None;
    let mut file_exist: Option<bool> = None;
    match lfile::check_file_exist(&info.hash, &app_config.file_storage_path).await {
        Ok(is_exist) => {
            file_exist = Some(is_exist);
        }
        Err(rsc) => {
            code = rsc.as_code();
            msg = Some(rsc.as_str().to_string());
        }
    }
    Ok(response_json::<Option<bool>>(code, msg, file_exist))
}

pub async fn create_ref_with_exist_file_and_storage(
    file_info: web::Json<FileExistInFolderInfo>,
    app_config: web::Data<AppConfig>,
    session: Session,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = match user_auth(session) {
        Ok(u_id) => u_id,
        Err(rsp) => {
            return Ok(rsp);
        }
    };
    let db_worker = db.get_free_worker().unwrap();
    let mut code: usize = 20000;
    let mut msg: Option<String> = None;
    let mut file_id: Option<String> = None;
    match lfile::ref_exist_file_and_storage(
        &user_id,
        &file_info.hash,
        &app_config.file_storage_path,
        &file_info.file_name,
        &file_info.storage_id,
        &db_worker.connection,
    )
    .await
    {
        Ok(f_id) => {
            file_id = Some(f_id);
        }
        Err(rsc) => {
            code = rsc.as_code();
            msg = Some(rsc.as_str().to_string());
        }
    }
    db.free(db_worker.index);

    Ok(response_json::<Option<String>>(code, msg, file_id))
}
