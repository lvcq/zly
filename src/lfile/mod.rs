use crate::lstorage;
use crate::yutils::current_naive_datetime;
use crate::yutils::short_id::generate_short_id;
use crate::zhttp::MultiFile;
use crate::zhttp::ResponseCode;
use crate::zpostgres::models::{FileStorage, ZlyFile};
use chrono::NaiveDateTime;
use diesel::expression_methods::BoolExpressionMethods;
use diesel::prelude::JoinOnDsl;
use diesel::Queryable;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Queryable)]
pub struct FileItem {
    file_id: String,
    file_name: String,
    file_mime: String,
    file_size: i64,
    created_time: NaiveDateTime,
}

pub async fn store_file(
    file_info: &MultiFile,
    storage_root_path: &str,
    user_id: String,
    storage_id: String,
    file_hash: &str,
    conn: &PgConnection,
) -> Result<String, ResponseCode> {
    let file_temp_path = file_info.temp_path.clone();
    let mut file_path = PathBuf::from(storage_root_path);
    file_path.push(file_hash);
    // 保存文件到文件夹
    if !is_storage_folder_container_file(&file_path) {
        save_file_to_folder(&file_path, &file_temp_path).await?;
    }
    // 存储到数据库，并获取文件ID
    let f_id = save_file_info(
        &user_id,
        file_hash,
        &file_info.filename,
        &file_info.mime,
        file_info.size,
        conn,
    )?;
    // 判断空间ID是否正确
    lstorage::is_storage_id_exist(&storage_id, conn)?;
    // 关联文件与存储仓库
    file_storage_ref(&f_id, &storage_id, conn)?;
    Ok(f_id)
}

pub async fn check_file_exist(hash: &str, storage_root_path: &str) -> Result<bool, ResponseCode> {
    let mut file_path = PathBuf::from(storage_root_path);
    file_path.push(&hash);
    Ok(is_storage_folder_container_file(&file_path))
}

fn is_storage_folder_container_file(file_path: &PathBuf) -> bool {
    if let Ok(file_meta) = fs::metadata(file_path) {
        if file_meta.is_file() {
            return true;
        }
        return false;
    } else {
        false
    }
}

/// 将文件存储到文件夹里
async fn save_file_to_folder(file_path: &PathBuf, temp_path: &str) -> Result<(), ResponseCode> {
    let file_path = file_path.clone();
    let temp_path = temp_path.to_string();
    match actix_web::web::block(|| fs::copy(temp_path, file_path)).await {
        Ok(_) => Ok(()),
        Err(_) => {
            return Err(ResponseCode::Code10003);
        }
    }
}

///
fn save_file_info(
    u_id: &str,
    hash: &str,
    filename: &str,
    mime: &str,
    size: i64,
    conn: &PgConnection,
) -> Result<String, ResponseCode> {
    return match search_file_info_by_user_id_and_hash_and_filename(u_id, hash, filename, conn) {
        Some(zf) => Ok(zf.file_id),
        None => {
            let f_id = insert_file_info_to_db(u_id, hash, filename, mime, size, conn)?;
            Ok(f_id)
        }
    };
}

/// 根据用户`ID`与文件`hash`查询文件

fn search_file_info_by_user_id_and_hash_and_filename(
    u_id: &str,
    hash: &str,
    filename: &str,
    conn: &PgConnection,
) -> Option<ZlyFile> {
    use crate::zpostgres::schema::zly_file::dsl::{file_hash, file_name, user_id, zly_file};
    let result: QueryResult<Vec<ZlyFile>> = zly_file
        .filter(file_hash.eq(hash))
        .filter(user_id.eq(u_id))
        .filter(file_name.eq(filename))
        .load::<ZlyFile>(conn);
    match result {
        Ok(mut zf_vec) => {
            if zf_vec.is_empty() {
                None
            } else {
                zf_vec.pop()
            }
        }
        Err(_) => None,
    }
}

/// 存储文件信息到数据库
fn insert_file_info_to_db(
    u_id: &str,
    hash: &str,
    filename: &str,
    mime: &str,
    size: i64,
    conn: &PgConnection,
) -> Result<String, ResponseCode> {
    use crate::zpostgres::schema::zly_file;
    let current = current_naive_datetime();
    let f_id = generate_short_id(16);
    let file_info = ZlyFile {
        created_time: current,
        user_id: u_id.to_string(),
        file_id: f_id.clone(),
        file_hash: hash.to_string(),
        file_name: filename.to_string(),
        file_size: size,
        file_mime: mime.to_string(),
    };
    let result = diesel::insert_into(zly_file::table)
        .values(&file_info)
        .get_result::<ZlyFile>(conn);
    return if result.is_err() {
        Err(ResponseCode::Code10003)
    } else {
        Ok(f_id)
    };
}
/// 关联空间和文件
fn file_storage_ref(f_id: &str, s_id: &str, conn: &PgConnection) -> Result<(), ResponseCode> {
    use crate::zpostgres::schema::file_storage;
    if let Ok(is_exist) = is_file_storage_ref_exist(f_id, s_id, conn) {
        if is_exist {
            return Ok(());
        }
    }
    let f_s_ref = FileStorage {
        storage_id: s_id.to_string(),
        file_id: f_id.to_string(),
    };
    let result: QueryResult<FileStorage> = diesel::insert_into(file_storage::table)
        .values(&f_s_ref)
        .get_result::<FileStorage>(conn);
    if result.is_err() {
        return Err(ResponseCode::Code10003);
    }
    Ok(())
}
/// 检验空间文件关联是否存在

fn is_file_storage_ref_exist(
    f_id: &str,
    s_id: &str,
    conn: &PgConnection,
) -> Result<bool, ResponseCode> {
    use crate::zpostgres::schema::file_storage::dsl::{file_id, file_storage, storage_id};
    let result: QueryResult<Vec<FileStorage>> = file_storage
        .filter(file_id.eq(f_id))
        .filter(storage_id.eq(s_id))
        .load::<FileStorage>(conn);
    if result.is_err() {
        return Err(ResponseCode::Code10003);
    }
    if result.unwrap().is_empty() {
        return Ok(false);
    } else {
        return Ok(true);
    }
}

/// 获取文件信息

fn get_file_info(hash: &str, conn: &PgConnection) -> Result<Option<Vec<ZlyFile>>, ResponseCode> {
    use crate::zpostgres::schema::zly_file::dsl::{file_hash, zly_file};
    let result: QueryResult<Vec<ZlyFile>> =
        zly_file.filter(file_hash.eq(hash)).load::<ZlyFile>(conn);
    if result.is_err() {
        return Err(ResponseCode::Code10003);
    }
    let res_vec = result.unwrap();
    return if res_vec.is_empty() {
        Ok(None)
    } else {
        Ok(Some(res_vec))
    };
}

/// 关联已经存在的文件和空间
pub async fn ref_exist_file_and_storage(
    u_id: &str,
    file_hash: &str,
    storage_root_path: &str,
    file_name: &str,
    storage_id: &str,
    conn: &PgConnection,
) -> Result<String, ResponseCode> {
    if check_file_exist(file_hash, storage_root_path).await? {
        if let Some(file_info_vec) = get_file_info(file_hash, conn)? {
            let file_info = file_info_vec.get(0).unwrap();
            let f_id = save_file_info(
                u_id,
                file_hash,
                file_name,
                &file_info.file_mime,
                file_info.file_size,
                conn,
            )?;
            // 判断空间ID是否正确
            lstorage::is_storage_id_exist(&storage_id, conn)?;
            // 关联文件与存储仓库
            file_storage_ref(&f_id, &storage_id, conn)?;
            return Ok(f_id);
        } else {
            return Err(ResponseCode::Code10003);
        }
    // 获取文件信息
    } else {
        return Err(ResponseCode::Code10003);
    }
}
/// 通过仓库ID查询文件列表
///
pub async fn query_file_list_by_storage_id(
    s_id: &str,
    conn: &PgConnection,
) -> Result<Vec<FileItem>, ResponseCode> {
    use crate::zpostgres::schema::file_storage;
    use crate::zpostgres::schema::zly_file;
    let file_list_query_res = file_storage::table
        .inner_join(
            zly_file::table.on(zly_file::file_id
                .eq(file_storage::file_id)
                .and(file_storage::storage_id.eq(s_id))),
        )
        .select((
            zly_file::file_id,
            zly_file::file_name,
            zly_file::file_mime,
            zly_file::file_size,
            zly_file::created_time,
        ))
        .load::<FileItem>(conn);
    match file_list_query_res {
        Ok(s_l) => Ok(s_l),
        Err(_) => Err(ResponseCode::Code10003),
    }
}
