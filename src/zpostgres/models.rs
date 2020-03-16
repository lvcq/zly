
#[derive(Queryable)]
#[table_name="user_info"]
pub struct UserInfo{
    pub user_id:String,
    pub user_name:String,
    pub password:String,
    pub create_time:NaiveDateTime,
    pub update_time:NaiveDateTime,
}
