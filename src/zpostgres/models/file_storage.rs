use diesel::{Queryable, Insertable};
use crate::zpostgres::schema::file_storage;


#[derive(Queryable, Insertable)]
#[table_name = "file_storage"]
pub struct FileStorage {
   pub storage_id:String,
   pub file_id: String,
}
