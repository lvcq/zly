use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable};
use crate::zpostgres::schema::user_info;


#[derive(Queryable, Insertable, Clone)]
#[table_name = "user_info"]
pub struct UserInfo {
    pub created_time: NaiveDateTime,
    pub user_name: String,
    pub updated_time: NaiveDateTime,
    pub user_id: String,
    pub password: String,
    pub email: Option<String>,
    pub last_login_time: NaiveDateTime,
}