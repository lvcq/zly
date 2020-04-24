use crate::yutils::current_naive_datetime;
use crate::yutils::short_id;
use crate::zhttp::response_code::ResponseCode;
use crate::zpostgres::models::storage::Storage;
use crate::zpostgres::models::{StorageRoleType, StorageUser};
use diesel::expression_methods::BoolExpressionMethods;
use diesel::prelude::JoinOnDsl;
use diesel::query_dsl::QueryDsl;
use diesel::Queryable;
use diesel::{ExpressionMethods, PgConnection, QueryResult, RunQueryDsl};
use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct StorageInfo {
    id: String,
    name: String,
    storage_role: StorageRoleType,
}

pub fn add_new_storage(
    user_id: String,
    storage_name: String,
    conn: &PgConnection,
) -> Result<(), ResponseCode> {
    use crate::zpostgres::schema::storage;
    let current = current_naive_datetime();
    let storage_id = short_id::generate_short_id(16);
    let storage = Storage {
        created_time: current.clone(),
        updated_time: current.clone(),
        storage_id: storage_id.clone(),
        storage_name,
        create_id: user_id.clone(),
    };
    let result = diesel::insert_into(storage::table)
        .values(&storage)
        .get_result::<Storage>(conn);
    if result.is_err() {
        return Err(ResponseCode::Code10003);
    }

    match ref_storage_add_user(user_id, storage_id.clone(), conn) {
        Ok(_) => Ok(()),
        Err(err) => {
            delete_storage_by_id(storage_id.clone(), conn);
            Err(err)
        }
    }
}

fn delete_storage_by_id(s_id: String, conn: &PgConnection) {
    use crate::zpostgres::schema::storage::dsl::{storage, storage_id};
    let result = diesel::delete(storage.filter(storage_id.eq(s_id))).get_result::<Storage>(conn);
    if result.is_err() {
        println!("delete err.");
    }
}

fn ref_storage_add_user(
    user_id: String,
    storage_id: String,
    conn: &PgConnection,
) -> Result<(), ResponseCode> {
    use crate::zpostgres::schema::storage_user;
    let current = current_naive_datetime();
    let storage_user = StorageUser {
        created_time: current.clone(),
        updated_time: current.clone(),
        user_id,
        storage_id,
        storage_role: StorageRoleType::Owner,
    };
    let result = diesel::insert_into(storage_user::table)
        .values(&storage_user)
        .get_result::<StorageUser>(conn);
    if result.is_err() {
        return Err(ResponseCode::Code10003);
    }
    Ok(())
}

pub fn get_storage_list_by_user_id(
    u_id: String,
    conn: &PgConnection,
) -> Result<Vec<StorageInfo>, ResponseCode> {
    use crate::zpostgres::schema::storage;
    use crate::zpostgres::schema::storage_user;
    let storage_list_result = storage_user::table
        .inner_join(
            storage::table.on(storage::storage_id
                .eq(storage_user::storage_id)
                .and(storage_user::user_id.eq(u_id))),
        )
        .select((
            storage::storage_id,
            storage::storage_name,
            storage_user::storage_role,
        ))
        .load::<StorageInfo>(conn);
    match storage_list_result {
        Ok(s_l) => Ok(s_l),
        Err(_) => Err(ResponseCode::Code10003),
    }
}

/// 校验仓库ID是否正确

pub fn is_storage_id_exist(s_id: &str, conn: &PgConnection) -> Result<(), ResponseCode> {
    use crate::zpostgres::schema::storage::dsl::{storage, storage_id};

    let result: QueryResult<Vec<Storage>> =
        storage.filter(storage_id.eq(s_id)).load::<Storage>(conn);
    if result.is_err() {
        return Err(ResponseCode::Code10003);
    }

    if result.unwrap().is_empty() {
        return Err(ResponseCode::Code10006);
    }

    Ok(())
}
