use super::schema::alert_source_info;

// Using #[derive(Queryable)] assumes that the order of fields in struct
// matches the columns in table, so make sure to define them in the order seen in the schema.rs file.
#[derive(Queryable, PartialEq, Clone)]
pub struct AlertSourceInfo {
    id: i32,
    pub source_type: String,
    pub identifier: String,
    pub connect_url: String,
    pub auth_type: String,
    pub connection_params: serde_json::Value,
    pub enabled: bool,
}

#[derive(Insertable)]
#[table_name="alert_source_info"]
pub struct NewAlertSourceInfo<'a> {
    pub source_type: &'a str,
    pub identifier: &'a str,
    pub connect_url: &'a str,
    pub auth_type: &'a str,
    pub connection_params: &'a serde_json::Value,
    pub enabled: &'a bool,
}