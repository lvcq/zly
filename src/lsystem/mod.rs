use super::zpostgres::models::{Role, UserRole};
use diesel::query_dsl::methods::FilterDsl;
use diesel::{RunQueryDsl, ExpressionMethods};
use diesel::PgConnection;
use crate::yutils::{short_id, current_naive_datetime};

pub fn is_init(conn: &PgConnection) -> bool {
    let role_id: String = match has_root_role(conn) {
        Some(id) => id,
        None => { return false; }
    };
    return match has_role_user_ref(role_id, conn) {
        Some(_) => true,
        None => false
    };
}

fn has_root_role(conn: &PgConnection) -> Option<String> {
    use super::zpostgres::schema::role::dsl::{role, role_name};
    let result: Vec<Role> = role.filter(role_name.eq("root"))
        .load::<Role>(conn).expect("加载角色信息失败");
    if result.len() == 0 {
        return None;
    } else {
        let first = result.get(0).unwrap();
        Some(first.role_id.clone())
    }
}

fn has_role_user_ref(r_id: String, conn: &PgConnection) -> Option<Vec<String>> {
    use super::zpostgres::schema::user_role::dsl::{user_role, role_id};
    let result: Vec<UserRole> = user_role.filter(role_id.eq(r_id))
        .load::<UserRole>(conn).expect("加载用户角色关联失败");
    if result.len() == 0 {
        return None;
    } else {
        let urv = result.iter().map(|ur| { ur.user_id.clone() }).collect();
        Some(urv)
    }
}


fn create_root_role(role_name: &str, conn: &PgConnection) -> bool {
    use super::zpostgres::schema::role;
    let current = current_naive_datetime();
    let role_id = short_id::generate_short_id(12);
    let root_role = Role {
        created_by: None,
        created_time: current.clone(),
        updated_by: None,
        updated_time: current,
        role_id,
        role_name: role_name.to_string(),
    };
    let result = diesel::insert_into(role::table)
        .values(&root_role).get_result::<Role>(conn);
    match result {
        Ok(_) => true,
        Err(_) => false
    }
}