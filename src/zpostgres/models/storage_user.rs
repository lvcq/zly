use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable, serialize, deserialize};
use diesel::sql_types::Text;
use diesel::pg::Pg;
use diesel::serialize::{Output, ToSql, IsNull};
use crate::zpostgres::schema::storage_user;
use std::io::Write;
use std::convert::Into;
use diesel::deserialize::FromSql;
use serde::Serialize;

#[derive(Queryable, Insertable)]
#[table_name = "storage_user"]
pub struct StorageUser {
    pub created_time: NaiveDateTime,
    pub updated_time: NaiveDateTime,
    pub user_id: String,
    pub storage_id: String,
    pub storage_role: StorageRoleType,
}


#[derive(Serialize, Debug, Copy, Clone, AsExpression, FromSqlRow)]
#[sql_type = "Text"]
pub enum StorageRoleType {
    Owner,
    Visitor,
    Maintainer,
}

impl ToSql<Text, Pg> for StorageRoleType {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            StorageRoleType::Owner => out.write_all(b"owner")?,
            StorageRoleType::Visitor => out.write_all(b"visitor")?,
            StorageRoleType::Maintainer => out.write_all(b"maintainer")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for StorageRoleType {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"owner" => Ok(StorageRoleType::Owner),
            b"visitor" => Ok(StorageRoleType::Visitor),
            b"maintainer" => Ok(StorageRoleType::Maintainer),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
