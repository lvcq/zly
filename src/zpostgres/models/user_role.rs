use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable};
use crate::zpostgres::schema::user_role;

#[derive(Queryable, Insertable)]
#[table_name = "user_role"]
pub struct UserRole {
    pub created_by: Option<String>,
    pub created_time: NaiveDateTime,
    pub updated_by: Option<String>,
    pub updated_time: NaiveDateTime,
    pub  id: String,
    pub user_id: String,
    pub role_id: String,
}
