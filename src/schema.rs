table! {
    alert_event (id) {
        id -> Int4,
    }
}

table! {
    alert_meta (id) {
        id -> Int4,
    }
}

table! {
    alert_source_info (id) {
        id -> Int4,
        source_type -> Varchar,
        identifier -> Varchar,
        connect_url -> Varchar,
        auth_type -> Varchar,
        connection_params -> Json,
        enabled -> Bool,
    }
}

table! {
    alert_tags (key, value) {
        id -> Int4,
        key -> Varchar,
        value -> Varchar,
    }
}

table! {
    groups (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    user_groups (user_id, group_id) {
        user_id -> Int4,
        group_id -> Int4,
    }
}

table! {
    user_notes (id) {
        id -> Int4,
        note -> Nullable<Text>,
        user_id -> Nullable<Int4>,
        alert_event_id -> Nullable<Int4>,
        visibility -> Varchar,
    }
}

table! {
    user_permissions (permission) {
        permission -> Varchar,
        enabled -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        is_active -> Bool,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        is_admin -> Bool,
        last_login -> Nullable<Timestamp>,
        date_joined -> Nullable<Timestamp>,
    }
}

joinable!(user_groups -> groups (group_id));
joinable!(user_groups -> users (user_id));
joinable!(user_notes -> alert_event (alert_event_id));
joinable!(user_notes -> users (user_id));

allow_tables_to_appear_in_same_query!(
    alert_event,
    alert_meta,
    alert_source_info,
    alert_tags,
    groups,
    user_groups,
    user_notes,
    user_permissions,
    users,
);
