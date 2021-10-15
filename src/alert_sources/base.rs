use crate::alert_sources::response::AlertList;
use std::error::Error;
// use crate::models::AlertSourceInfo;

pub trait AlertSource {
    /// This is the base trait for defining functions common for all alert sources
    /// Every Alert source must implement this trait

    // fn new(obj: AlertSourceInfo) -> Result<dyn AlertSource, Box<dyn Error>>;
    fn get_source_name(&self) -> &str;
    fn get_active_alerts(&mut self) -> Result<AlertList, Box<dyn Error>>; // mutable reference is required to set the auth key
    fn acknowledge_alert(&self) -> bool;
}
