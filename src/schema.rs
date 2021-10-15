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
