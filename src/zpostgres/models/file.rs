use crate::zpostgres::schema::zly_file;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};

#[derive(Queryable, Insertable)]
#[table_name = "zly_file"]
pub struct ZlyFile {
    pub created_time: NaiveDateTime,
    pub user_id: String,
    pub file_id: String,
    pub file_hash: String,
    pub file_name: String,
    pub file_size: i64,
    pub file_mime: String,
}
