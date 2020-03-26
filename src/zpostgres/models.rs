use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable};
use super::schema::role;

#[derive(Queryable)]
pub struct UserInfo {
    pub user_id: String,
    pub user_name: String,
    pub password: String,
    pub email: Option<String>,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

#[derive(Queryable, Insertable)]
#[table_name="role"]
pub struct Role {
    pub created_by: Option<String>,
    pub created_time: NaiveDateTime,
    pub updated_by: Option<String>,
    pub updated_time: NaiveDateTime,
    pub role_id: String,
    pub role_name: String,
}


#[derive(Queryable)]
pub struct UserRole {
    pub created_by: Option<String>,
    pub created_time: NaiveDateTime,
    pub updated_by: Option<String>,
    pub updated_time: NaiveDateTime,
    pub  id: String,
    pub user_id: String,
    pub role_id: String,
}