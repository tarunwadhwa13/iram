use crate::alert_sources::response::AlertList;
use crate::errors::UnsupportedError;
use crate::models::AlertSourceInfo;
use std::error::Error;

/// This is the base trait for defining functions common for all alert sources.
/// Every Alert source must implement this trait
pub trait AlertSource {
    /// function for creating alert source object from database returned params
    fn new_from_object(obj: &AlertSourceInfo) -> Self
    where
        Self: std::marker::Sized;

    /// Should return source for the alert. This can any identifier for alert source type
    fn get_source_name(&self) -> &str;

    /// Used to test connection to the alert source. Connection testing can vary from ensuring connectivity to verifying credentials
    fn test_connection(&mut self) -> bool;

    /// function used for processing webhook received. This is responsible for processing webhook and returning alert list.
    /// Either this or get_active_alerts should be implemented
    fn process_webhook(&self) -> Result<AlertList, Box<dyn Error>> {
        return Err(Box::new(UnsupportedError(
            "Webhook processing is not supported for this source".to_string(),
        )));
    }

    /// Used for sources where webhook is not possible. This should also return a list of alerts
    fn get_active_alerts(&mut self) -> Result<AlertList, Box<dyn Error>> {
        return Err(Box::new(UnsupportedError(
            "The method is not implemented for this source".to_string(),
        )));
    }

    /// Optional. This should be implemented if source supports acknowledgments
    fn acknowledge_alert(&self) -> bool {
        true
    }

    /// This function is called for writing alert list to DB
    /// Adding a default function here. Overriding this is possible but should be avoided.
    fn add_alert_to_db(&self, alerts: AlertList) -> Result<bool, Box<dyn Error>> {
        Ok(true)
    }
}

/// This is a common function for extracting alerts from database.
fn get_alerts_from_db() {
    todo!()
}
