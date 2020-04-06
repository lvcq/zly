use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable};
use crate::zpostgres::schema::role;

#[derive(Queryable, Insertable)]
#[table_name = "role"]
pub struct Role {
    pub created_by: Option<String>,
    pub created_time: NaiveDateTime,
    pub updated_by: Option<String>,
    pub updated_time: NaiveDateTime,
    pub role_id: String,
    pub role_name: String,
}