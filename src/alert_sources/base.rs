use crate::alert_sources::response::AlertList;
use crate::models::AlertSourceInfo;
use std::error::Error;

pub trait AlertSource {
    /// This is the base trait for defining functions common for all alert sources
    /// Every Alert source must implement this trait

    fn new_from_object(obj: &AlertSourceInfo) -> Result<Self, Box<dyn Error>>
    where
        Self: std::marker::Sized;

    fn get_source_name(&self) -> &str;

    fn process_webhook(&self) -> Result<AlertList, Box<dyn Error>> {
        Ok(Vec::new())
    } // function used for processing webhook received

    fn get_active_alerts(&mut self) -> Result<AlertList, Box<dyn Error>> {
        Ok(Vec::new())
    } // Used for sources where webhook is not possible

    fn acknowledge_alert(&self) -> bool;

    // Adding a default function here. Overriding this is possible but should be avoided
    fn add_alert_to_db(&self, alerts: AlertList) -> Result<bool, Box<dyn Error>> {
        Ok(true)
    }
}

fn get_alerts_from_db() {}
