use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Alert {
    pub source: String,
    pub event_id: String,
    pub trigger_id: String,
    pub subject: String,
    pub entity: String,
    pub groups: Vec<String>,
    pub alert_start_time: String,
    pub alert_age: String,
    pub alert_status: String,
    pub priority: String,
    pub tags: HashMap<String, String>
}

pub type AlertList = Vec<Alert>;

// 'description': entry['description'],
// 'alert_time': alert_age,
// 'lastchange_utc': last_change,
// 'ack': ack_status,
// 'sendto': ",".join(send_to_list),
// 'subject': subject,
// 'message': message,