#[derive(Debug)]
pub struct Alert {
    pub source: String,
    pub event_id: u64,
    pub entity: String,
    pub alert_start_time: String,
    pub alert_status: String,
    pub priority: String,
}

pub type AlertList = Vec<Alert>;