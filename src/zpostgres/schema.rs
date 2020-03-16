table! {
    menu (id) {
        created_by -> Nullable<Varchar>,
        created_time -> Nullable<Date>,
        updated_by -> Nullable<Varchar>,
        updated_time -> Nullable<Date>,
        id -> Varchar,
        menu_name -> Varchar,
        menu_url -> Nullable<Varchar>,
        menu_p_id -> Nullable<Varchar>,
    }
}

table! {
    opetation (id) {
        created_by -> Nullable<Varchar>,
        created_time -> Nullable<Date>,
        updated_by -> Nullable<Varchar>,
        updated_time -> Nullable<Date>,
        id -> Varchar,
        opt_name -> Varchar,
        opt_code -> Nullable<Varchar>,
        opt_p_id -> Nullable<Varchar>,
    }
}

table! {
    page_element (element_id) {
        created_by -> Nullable<Varchar>,
        created_time -> Nullable<Date>,
        updated_by -> Nullable<Varchar>,
        updated_time -> Nullable<Date>,
        element_id -> Varchar,
        element_name -> Varchar,
    }
}

table! {
    pdman_db_version (db_version) {
        db_version -> Varchar,
        version_desc -> Nullable<Varchar>,
        created_time -> Nullable<Varchar>,
    }
}

table! {
    power (id) {
        created_by -> Nullable<Varchar>,
        created_time -> Nullable<Date>,
        updated_by -> Nullable<Varchar>,
        updated_time -> Nullable<Date>,
        id -> Varchar,
        power_type -> Varchar,
        power_code -> Nullable<Varchar>,
    }
}

table! {
    power_menu (id) {
        created_by -> Nullable<Varchar>,
        created_time -> Nullable<Date>,
        updated_by -> Nullable<Varchar>,
        updated_time -> Nullable<Date>,
        id -> Varchar,
        menu_id -> Varchar,
        power_id -> Varchar,
    }
}

table! {
    power_opt (id) {
        created_by -> Nullable<Varchar>,
        created_time -> Nullable<Date>,
        updated_by -> Nullable<Varchar>,
        updated_time -> Nullable<Date>,
        id -> Varchar,
        power_id -> Varchar,
        opt_id -> Varchar,
    }
}

table! {
    power_page_element (id) {
        created_by -> Nullable<Varchar>,
        created_time -> Nullable<Date>,
        updated_by -> Nullable<Varchar>,
        updated_time -> Nullable<Date>,
        id -> Varchar,
        power_id -> Varchar,
        element_id -> Varchar,
    }
}

table! {
    role (role_id) {
        created_by -> Nullable<Varchar>,
        created_time -> Nullable<Date>,
        updated_by -> Nullable<Varchar>,
        updated_time -> Nullable<Date>,
        role_id -> Varchar,
        role_name -> Varchar,
    }
}

table! {
    role_power (id) {
        created_by -> Nullable<Varchar>,
        created_time -> Nullable<Date>,
        updated_by -> Nullable<Varchar>,
        updated_time -> Nullable<Date>,
        id -> Varchar,
        role_id -> Varchar,
        power_id -> Varchar,
    }
}

table! {
    user_group (user_group_id) {
        created_by -> Nullable<Varchar>,
        created_time -> Nullable<Date>,
        updated_by -> Nullable<Varchar>,
        updated_time -> Nullable<Date>,
        user_group_id -> Varchar,
        user_group_name -> Nullable<Varchar>,
    }
}

table! {
    user_group_role (user_group_id, role_id) {
        created_by -> Nullable<Varchar>,
        created_time -> Nullable<Date>,
        updated_by -> Nullable<Varchar>,
        updated_time -> Nullable<Date>,
        id -> Varchar,
        user_group_id -> Varchar,
        role_id -> Varchar,
    }
}

table! {
    user_group_user_info (user_group_id, user_id) {
        created_by -> Nullable<Varchar>,
        created_time -> Nullable<Date>,
        updated_by -> Nullable<Varchar>,
        updated_time -> Nullable<Date>,
        id -> Varchar,
        user_group_id -> Varchar,
        user_id -> Varchar,
    }
}

table! {
    user_info (user_id) {
        created_time -> Nullable<Date>,
        user_name -> Varchar,
        updated_time -> Nullable<Date>,
        user_id -> Varchar,
        password -> Varchar,
        email -> Nullable<Varchar>,
    }
}

table! {
    user_role (id) {
        created_by -> Nullable<Varchar>,
        created_time -> Nullable<Date>,
        updated_by -> Nullable<Varchar>,
        updated_time -> Nullable<Date>,
        id -> Varchar,
        user_id -> Varchar,
        role_id -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    menu,
    opetation,
    page_element,
    pdman_db_version,
    power,
    power_menu,
    power_opt,
    power_page_element,
    role,
    role_power,
    user_group,
    user_group_role,
    user_group_user_info,
    user_info,
    user_role,
);
