use diesel::query_dsl::methods::FilterDsl;
use diesel::{RunQueryDsl, ExpressionMethods};
use diesel::PgConnection;
use serde::Serialize;
use crate::zhttp::response_code::ResponseCode;
use crate::zpostgres::models::user_info::UserInfo;
use crate::yutils::crypto_password_with_username_timestamp;

#[derive(Serialize)]
pub struct ShowUserInfo {
    pub(crate)  username: String,
    pub(crate)  email: Option<String>,
    pub(crate)  last_login_time: Option<u64>,
}

pub fn validate_user(in_user_id: String, conn: &PgConnection) -> Result<ShowUserInfo, ResponseCode> {
    use crate::zpostgres::schema::user_info::dsl::{user_info, user_id, user_name, email};
    let result: Vec<UserInfo> = user_info.filter(user_id.eq(in_user_id))
        .load::<UserInfo>(conn).expect("查询用户信息失败");
    if result.len() == 0 {
        return Err(ResponseCode::Code10002);
    }
    return Ok(ShowUserInfo {
        username: result[0].user_name.clone(),
        email: result[0].email.clone(),
        last_login_time:None
    });
}

pub fn validate_user_password(username: &str, sha_pwd: &str, timestamp: u64, conn: &PgConnection) -> Result<UserInfo, ResponseCode> {
    use crate::zpostgres::schema::user_info::dsl::{user_info, user_name, password};
    let result: Vec<UserInfo> = match user_info.filter(user_name.eq(username)).load::<UserInfo>(conn) {
        Ok(res) => res,
        Err(_) => {
            return Err(ResponseCode::Code10003);
        }
    };
    if result.len() == 0 {
        return Err(ResponseCode::Code10004);
    }
    let u_pwd = result[0].password.clone();
    let u_name = result[0].user_name.clone();
    return if crypto_password_with_username_timestamp(&u_pwd, &u_name, timestamp, sha_pwd) {
        Ok(result.first().unwrap().clone())
    } else {
        Err(ResponseCode::Code10004)
    };
}