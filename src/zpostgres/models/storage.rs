use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable};
use crate::zpostgres::schema::storage;

#[derive(Queryable, Insertable)]
#[table_name = "storage"]
pub struct Storage {
    pub created_time: NaiveDateTime,
    pub updated_time: NaiveDateTime,
    pub storage_id: String,
    pub storage_name: String,
    pub create_id: String,
}