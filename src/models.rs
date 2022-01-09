use super::schema::alert_source_info;
use chrono::{DateTime, Utc};

// Using #[derive(Queryable)] assumes that the order of fields in struct
// matches the columns in table, so make sure to define them in the order seen in the schema.rs file.
#[derive(Queryable, PartialEq, Clone)]
pub struct AlertSourceInfo {
    pub id: i32,
    pub source_type: String,
    pub identifier: String,
    pub connect_url: String,
    pub auth_type: String,
    pub connection_params: serde_json::Value,
    pub enabled: bool,
}

#[derive(Insertable)]
#[table_name = "alert_source_info"]
pub struct NewAlertSourceInfo<'a> {
    pub source_type: &'a str,
    pub identifier: &'a str,
    pub connect_url: &'a str,
    pub auth_type: &'a str,
    pub connection_params: &'a serde_json::Value,
    pub enabled: &'a bool,
}

#[derive(Queryable, PartialEq, Clone)]
pub struct Alerts {
    pub id: i32,
    pub source_id: i32,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub state: String,
    pub assigned_user_id: Option<i32>,
    pub subject: String,
    pub description: String,
    pub priority: String,
    pub entity: String,
    pub entity_group: String,
    pub timeout: i32,
    pub first_callback_at: Option<DateTime<Utc>>,
    pub last_callback_at: Option<DateTime<Utc>>,
}

#[derive(Queryable, PartialEq, Clone)]
pub struct Users {
    pub id: i32,
    pub username: String,
    password: String,
    pub is_active: bool,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub is_admin: bool,
    pub last_login: Option<DateTime<Utc>>,
    pub date_joined: Option<DateTime<Utc>>
}
