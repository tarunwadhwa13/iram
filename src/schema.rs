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
    alert_tags (alert_id, tag_id) {
        alert_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    alert_timeline (id) {
        id -> Int4,
        event_type -> Varchar,
        reported_at -> Timestamptz,
        event_info -> Varchar,
    }
}

table! {
    alerts (id) {
        id -> Int4,
        source_id -> Int4,
        created_at -> Timestamptz,
        last_updated -> Timestamptz,
        state -> Varchar,
        assigned_user_id -> Nullable<Int4>,
        subject -> Varchar,
        description -> Varchar,
        priority -> Varchar,
        entity -> Varchar,
        entity_group -> Varchar,
        timeout -> Int4,
        first_callback_at -> Nullable<Timestamptz>,
        last_callback_at -> Nullable<Timestamptz>,
    }
}

table! {
    group_permissions (group_id, permission_key) {
        group_id -> Int4,
        permission_key -> Varchar,
        enabled -> Bool,
    }
}

table! {
    groups (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    incident_alert (incident_id, alert_id) {
        incident_id -> Int4,
        alert_id -> Int4,
    }
}

table! {
    incident_report (id) {
        id -> Int4,
        segments_lost -> Numeric,
        loss_details -> Varchar,
        cost -> Numeric,
        acked_at -> Timestamptz,
        resolved_at -> Nullable<Timestamptz>,
        status -> Varchar,
        resolution -> Varchar,
        created_at -> Timestamptz,
        last_updated -> Timestamptz,
    }
}

table! {
    permission (key) {
        key -> Varchar,
        description -> Varchar,
    }
}

table! {
    tags (id) {
        id -> Int4,
        key -> Varchar,
        value -> Varchar,
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
        note -> Varchar,
        user_id -> Nullable<Int4>,
        alert_id -> Nullable<Int4>,
        visibility -> Varchar,
        created_at -> Nullable<Timestamptz>,
        last_updated -> Nullable<Timestamptz>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        is_active -> Bool,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        is_admin -> Bool,
        last_login -> Nullable<Timestamptz>,
        date_joined -> Nullable<Timestamptz>,
    }
}

joinable!(alert_tags -> alerts (alert_id));
joinable!(alert_tags -> tags (tag_id));
joinable!(alerts -> alert_source_info (source_id));
joinable!(alerts -> users (assigned_user_id));
joinable!(group_permissions -> groups (group_id));
joinable!(incident_alert -> alerts (alert_id));
joinable!(incident_alert -> incident_report (incident_id));
joinable!(user_groups -> groups (group_id));
joinable!(user_groups -> users (user_id));
joinable!(user_notes -> alerts (alert_id));
joinable!(user_notes -> users (user_id));

allow_tables_to_appear_in_same_query!(
    alert_source_info,
    alert_tags,
    alert_timeline,
    alerts,
    group_permissions,
    groups,
    incident_alert,
    incident_report,
    permission,
    tags,
    user_groups,
    user_notes,
    users,
);
