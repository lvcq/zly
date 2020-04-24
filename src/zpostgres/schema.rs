table! {
    file_storage (storage_id, file_id) {
        storage_id -> Varchar,
        file_id -> Varchar,
    }
}

table! {
    menu (id) {
        created_by -> Nullable<Varchar>,
        created_time -> Timestamp,      
        updated_by -> Nullable<Varchar>,
        updated_time -> Timestamp,
        id -> Varchar,
        menu_name -> Varchar,
        menu_url -> Nullable<Varchar>,
        menu_p_id -> Nullable<Varchar>,
    }
}

table! {
    opetation (id) {
        created_by -> Nullable<Varchar>,
        created_time -> Timestamp,
        updated_by -> Nullable<Varchar>,
        updated_time -> Timestamp,
        id -> Varchar,
        opt_name -> Varchar,
        opt_code -> Nullable<Varchar>,
        opt_p_id -> Nullable<Varchar>,
    }
}

table! {
    page_element (element_id) {
        created_by -> Nullable<Varchar>,
        created_time -> Timestamp,
        updated_by -> Nullable<Varchar>,
        updated_time -> Timestamp,
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
        created_time -> Timestamp,
        updated_by -> Nullable<Varchar>,
        updated_time -> Timestamp,
        id -> Varchar,
        power_type -> Varchar,
        power_code -> Nullable<Varchar>,
    }
}

table! {
    power_menu (id) {
        created_by -> Nullable<Varchar>,
        created_time -> Timestamp,
        updated_by -> Nullable<Varchar>,
        updated_time -> Timestamp,
        id -> Varchar,
        menu_id -> Varchar,
        power_id -> Varchar,
    }
}

table! {
    power_opt (id) {
        created_by -> Nullable<Varchar>,
        created_time -> Timestamp,
        updated_by -> Nullable<Varchar>,
        updated_time -> Timestamp,
        id -> Varchar,
        power_id -> Varchar,
        opt_id -> Varchar,
    }
}

table! {
    power_page_element (id) {
        created_by -> Nullable<Varchar>,
        created_time -> Timestamp,
        updated_by -> Nullable<Varchar>,
        updated_time -> Timestamp,
        id -> Varchar,
        power_id -> Varchar,
        element_id -> Varchar,
    }
}

table! {
    role (role_id) {
        created_by -> Nullable<Varchar>,
        created_time -> Timestamp,
        updated_by -> Nullable<Varchar>,
        updated_time -> Timestamp,
        role_id -> Varchar,
        role_name -> Varchar,
    }
}

table! {
    role_power (id) {
        created_by -> Nullable<Varchar>,
        created_time -> Timestamp,
        updated_by -> Nullable<Varchar>,
        updated_time -> Timestamp,
        id -> Varchar,
        role_id -> Varchar,
        power_id -> Varchar,
    }
}

table! {
    storage (storage_id) {
        created_time -> Timestamp,
        updated_time -> Timestamp,
        storage_id -> Varchar,
        storage_name -> Varchar,
        create_id -> Varchar,
    }
}

table! {
    storage_user (user_id, storage_id) {
        created_time -> Timestamp,
        updated_time -> Timestamp,
        user_id -> Varchar,
        storage_id -> Varchar,
        storage_role -> Varchar,
    }
}

table! {
    user_group (user_group_id) {
        created_by -> Nullable<Varchar>,
        created_time -> Timestamp,
        updated_by -> Nullable<Varchar>,
        updated_time -> Timestamp,
        user_group_id -> Varchar,
        user_group_name -> Nullable<Varchar>,
    }
}

table! {
    user_group_role (user_group_id, role_id) {
        created_by -> Nullable<Varchar>,
        created_time -> Timestamp,
        updated_by -> Nullable<Varchar>,
        updated_time -> Timestamp,
        id -> Varchar,
        user_group_id -> Varchar,
        role_id -> Varchar,
    }
}

table! {
    user_group_user_info (user_group_id, user_id) {
        created_by -> Nullable<Varchar>,
        created_time -> Timestamp,
        updated_by -> Nullable<Varchar>,
        updated_time -> Timestamp,
        id -> Varchar,
        user_group_id -> Varchar,
        user_id -> Varchar,
    }
}

table! {
    user_info (user_id) {
        created_time -> Timestamp,
        user_name -> Varchar,
        updated_time -> Timestamp,
        user_id -> Varchar,
        password -> Varchar,
        email -> Nullable<Varchar>,
        last_login_time -> Timestamp,
    }
}

table! {
    user_role (id) {
        created_by -> Nullable<Varchar>,
        created_time -> Timestamp,
        updated_by -> Nullable<Varchar>,
        updated_time -> Timestamp,
        id -> Varchar,
        user_id -> Varchar,
        role_id -> Varchar,
    }
}

table! {
    zly_file (file_id) {
        created_time -> Timestamp,
        user_id -> Varchar,
        file_id -> Varchar,
        file_hash -> Varchar,
        file_name -> Varchar,
        file_size -> Int8,
        file_mime -> Varchar,
    }
}

joinable!(file_storage -> storage (storage_id));
joinable!(file_storage -> zly_file (file_id));
joinable!(storage_user -> storage (storage_id));
joinable!(storage_user -> user_info (user_id));
joinable!(zly_file -> user_info (user_id));

allow_tables_to_appear_in_same_query!(
    file_storage,
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
    storage,
    storage_user,
    user_group,
    user_group_role,
    user_group_user_info,
    user_info,
    user_role,
    zly_file,
);