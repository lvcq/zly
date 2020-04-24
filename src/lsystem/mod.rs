use crate::zpostgres::models::{role::Role, user_role::UserRole, user_info::UserInfo};
use diesel::query_dsl::methods::FilterDsl;
use diesel::{RunQueryDsl, ExpressionMethods};
use diesel::PgConnection;
use crate::yutils::{short_id, current_naive_datetime, crypto_password};
use serde::Deserialize;
use actix_web::web;
use crate::zhttp::response_code::ResponseCode;

#[derive(Deserialize)]
pub struct RootInfo {
    pub root_name: String,
    pub password: String,
    pub email: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginInfo {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) timestamp: u64,
}

pub fn is_init(conn: &PgConnection) -> bool {
    let role_id: String = match has_root_role(conn) {
        Some(id) => id,
        None => { return false; }
    };
    return match has_role_user_ref(role_id, conn) {
        Some(_) => true,
        None => false
    };
}

fn has_root_role(conn: &PgConnection) -> Option<String> {
    use super::zpostgres::schema::role::dsl::{role, role_name};
    let result: Vec<Role> = role.filter(role_name.eq("root"))
        .load::<Role>(conn).expect("加载角色信息失败");
    if result.len() == 0 {
        return None;
    } else {
        let first = result.get(0).unwrap();
        Some(first.role_id.clone())
    }
}

fn has_role_user_ref(r_id: String, conn: &PgConnection) -> Option<Vec<String>> {
    use super::zpostgres::schema::user_role::dsl::{user_role, role_id};
    let result: Vec<UserRole> = user_role.filter(role_id.eq(r_id))
        .load::<UserRole>(conn).expect("加载用户角色关联失败");
    if result.len() == 0 {
        return None;
    } else {
        let urv = result.iter().map(|ur| { ur.user_id.clone() }).collect();
        Some(urv)
    }
}


fn create_root_role(conn: &PgConnection) -> Result<String, ResponseCode> {
    use super::zpostgres::schema::role;
    let current = current_naive_datetime();
    let role_id = short_id::generate_short_id(12);
    let root_role = Role {
        created_by: None,
        created_time: current.clone(),
        updated_by: None,
        updated_time: current,
        role_id: role_id.clone(),
        role_name: "root".to_string(),
    };
    let result = diesel::insert_into(role::table)
        .values(&root_role).get_result::<Role>(conn);
    match result {
        Ok(_) => Ok(role_id),
        Err(_) => Err(ResponseCode::Code10001)
    }
}

fn create_root_user(root_name: &str,
                    password: &str,
                    email: Option<String>,
                    conn: &PgConnection) -> Result<String, ResponseCode> {
    use super::zpostgres::schema::user_info;
    let current = current_naive_datetime();
    let user_id = short_id::generate_short_id(12);
    let user_info_ins = UserInfo {
        user_id: user_id.clone(),
        user_name: root_name.to_string(),
        password: crypto_password(password),
        email: email.clone(),
        created_time: current.clone(),
        updated_time: current.clone(),
        last_login_time: current.clone(),
    };
    match diesel::insert_into(user_info::table)
        .values(&user_info_ins).get_result::<UserInfo>(conn) {
        Ok(_) => Ok(user_id),
        Err(_) => Err(ResponseCode::Code10001)
    }
}

fn create_root_user_role_ref(user_id: String, role_id: String, conn: &PgConnection) -> Result<(), ResponseCode> {
    use super::zpostgres::schema::user_role;
    let current = current_naive_datetime();
    let row_id = short_id::generate_short_id(12);
    let user_role_ins = UserRole {
        created_by: None,
        created_time: current.clone(),
        updated_by: None,
        updated_time: current.clone(),
        id: row_id,
        user_id,
        role_id,
    };
    match diesel::insert_into(user_role::table)
        .values(&user_role_ins).get_result::<UserRole>(conn) {
        Ok(_) => Ok(()),
        Err(_) => Err(ResponseCode::Code10001)
    }
}

pub fn set_root_info(root_info: web::Json<RootInfo>, conn: &PgConnection) -> Result<bool, ResponseCode> {
    if is_init(conn) {
        println!("false");
        return Err(ResponseCode::Code10001);
    }
    println!("true");
    let role_id = create_root_role(conn)?;
    let user_id = create_root_user(
        &root_info.root_name,
        &root_info.password,
        root_info.email.clone(),
        conn)?;
    create_root_user_role_ref(user_id, role_id, conn)?;
    Ok(true)
}