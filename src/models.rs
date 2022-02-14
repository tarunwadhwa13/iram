use super::schema::alert_source_info;
use bigdecimal::BigDecimal;
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
    pub enabled: bool,
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
    last_login: Option<DateTime<Utc>>,
    date_joined: Option<DateTime<Utc>>,
}

impl Users {
    pub fn get_password(&self) -> String {
        self.password.clone()
    }
    pub fn last_login(&self) -> Option<DateTime<Utc>> {
        self.last_login
    }
    pub fn date_joined(&self) -> Option<DateTime<Utc>> {
        self.date_joined
    }

    pub fn last_login_str(&self) -> Option<String> {
        match self.last_login() {
            Some(result) => return Some(result.to_rfc3339()),
            None => return None,
        };
    }

    pub fn date_joined_str(&self) -> Option<String> {
        match self.date_joined() {
            Some(result) => return Some(result.to_rfc3339()),
            None => return None,
        };
    }
}

#[derive(Queryable, PartialEq, Clone)]
pub struct IncidentReport {
    pub id: i32,
    pub segments_lost: BigDecimal,
    pub loss_details: String,
    pub cost: BigDecimal,
    pub acked_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub status: String,
    pub resolution: String,
    created_at: DateTime<Utc>,
    last_updated: DateTime<Utc>,
}

impl IncidentReport {
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn last_updated(&self) -> DateTime<Utc> {
        self.last_updated
    }

    pub fn created_at_str(&self) -> String {
        self.created_at().to_rfc3339()
    }

    pub fn last_updated_str(&self) -> String {
        self.last_updated().to_rfc3339()
    }

    pub fn acked_at_str(&self) -> String {
        self.acked_at.to_rfc3339()
    }

    pub fn resolved_at_str(&self) -> Option<String> {
        match self.resolved_at {
            Some(value) => Some(value.to_rfc3339()),
            None => None,
        }
    }
}

#[derive(Queryable, PartialEq, Clone)]
pub struct IncidentAlert {
    pub incident_id: i32,
    pub alert_id: i32,
}
