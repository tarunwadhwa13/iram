use std::fmt;

use std::error::Error;
use actix_web::{error, Result};

// Zabbix Error
#[derive(Debug)]
pub struct ZabbixError(pub String); // used in zabbix mod

#[derive(Debug)]
pub struct DBConnectionError(pub String); // used in DB module

#[derive(Debug)]
pub struct GenericError(pub String); // used as a generic exception

#[derive(Debug)]
pub struct GenericAlertSourceError(pub String); // used as a generic exception

#[derive(Debug)]
pub struct NewRelicError(pub String); // used in newrelic mod

#[derive(Debug)]
pub struct ManageEngineError(pub String); // used in manage engine mod

#[derive(Debug)]
pub struct SettingsError(pub String); // used in settings module

#[derive(Debug)]
pub struct UnsupportedError(pub String); // used when attribute is unsupported


impl fmt::Display for ZabbixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ZabbixError: {}", self.0)
    }
}

impl fmt::Display for DBConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DBConnectionError: {}", self.0)
    }
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UnknownError: {}", self.0)
    }
}

impl fmt::Display for GenericAlertSourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GenericAlertSourceError: {}", self.0)
    }
}

impl fmt::Display for NewRelicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NewRelicError: {}", self.0)
    }
}

impl fmt::Display for ManageEngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ManageEngineError: {}", self.0)
    }
}

impl fmt::Display for SettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SettingsError: {}", self.0)
    }
}

impl fmt::Display for UnsupportedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UnsupportedError: {}", self.0)
    }
}

impl Error for ZabbixError {}
impl Error for DBConnectionError {}
impl Error for GenericError {}
impl Error for GenericAlertSourceError {}
impl Error for UnsupportedError {}
impl Error for NewRelicError {}
impl Error for ManageEngineError {}
impl Error for SettingsError {}

impl error::ResponseError for ZabbixError {}
impl error::ResponseError for DBConnectionError {}
impl error::ResponseError for GenericError {}
impl error::ResponseError for GenericAlertSourceError {}
impl error::ResponseError for UnsupportedError {}
impl error::ResponseError for NewRelicError {}
impl error::ResponseError for ManageEngineError {}
impl error::ResponseError for SettingsError {}