pub mod base;
pub mod response;
pub mod zabbix;

use crate::alert_sources::base::AlertSource;
use std::error::Error;

use crate::db::get_connection;
use crate::errors::{GenericAlertSourceError, UnsupportedError};

use crate::schema::alert_source_info::dsl;
use diesel::{ExpressionMethods, RunQueryDsl};
use crate::diesel::query_dsl::filter_dsl::FilterDsl;
use crate::models::AlertSourceInfo;


pub fn get_alert_source_handler(source: &str, identifier: &str) -> Result<impl AlertSource, Box<dyn Error>> {

    let connection = get_connection().unwrap();

    let query_response = dsl::alert_source_info.filter(dsl::source_type.eq(source))
                        .filter(dsl::identifier.eq(identifier))
                        .filter(dsl::enabled.eq(true))
                        .load::<AlertSourceInfo>(&connection)
                        .expect("Error loading alert source");
    
    log::info!("Got {} alert sources", query_response.len());

    if query_response.len() == 0 {
        let err_msg = format!("No enabled source found with identifier - {}, source_type - {}", identifier, source);
        return Err(Box::new(GenericAlertSourceError(err_msg.to_string())));
    } else {
        let alert_source_details = &query_response[0];
        match source {
            "Zabbix" => {
                return zabbix::zabbix::ZabbixHandler::new_from_object(alert_source_details);
            },
            _ => {
                let err_msg = format!("The source type - {} is not supported yet", source);
                return Err(Box::new(UnsupportedError(err_msg.to_string())));
            }
        }
    }
}