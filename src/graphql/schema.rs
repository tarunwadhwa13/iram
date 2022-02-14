use super::controller;
use super::defs::{
    GQLAlertEvent, GQLAlertList, GQLAlertSource, GQLAlertSourceList, GQLIncidentReport,
    GQLIncidentReportList, GQLNewAlertSource,
};
use crate::models::NewAlertSourceInfo;
use bigdecimal::ToPrimitive;
use juniper::{EmptySubscription, RootNode};
use juniper::{FieldError, FieldResult, Value as juniperValue};
use serde_json::Value as jsonValue;
use std::collections::HashMap;

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn alert_sources() -> FieldResult<GQLAlertSourceList> {
        let query_response = controller::get_alert_sources();

        let mut alert_source_list: GQLAlertSourceList = Vec::new();
        for entry in query_response.into_iter() {
            alert_source_list.push(GQLAlertSource {
                id: entry.id,
                source_type: entry.source_type.to_string(),
                identifier: entry.identifier.to_string(),
                connect_url: entry.connect_url.to_string(),
                auth_type: entry.auth_type.to_string(),
                enabled: entry.enabled,
            });
        }
        return Ok(alert_source_list);
    }

    fn active_alerts() -> FieldResult<GQLAlertList> {
        let query_response = controller::get_active_alerts();

        let mut alert_list: GQLAlertList = Vec::new();
        for entry in query_response.into_iter() {
            let alert = entry.0;
            let alert_source = entry.1;
            let user = entry.2;
            let event = GQLAlertEvent::generate_from_db_objects(alert, alert_source, user);
            alert_list.push(event);
        }
        return Ok(alert_list);
    }

    fn past_alerts(_time_limit: i32) -> FieldResult<GQLAlertList> {
        let query_response = controller::get_past_alerts();

        let mut alert_list: GQLAlertList = Vec::new();
        for entry in query_response.into_iter() {
            let alert = entry.0;
            let alert_source = entry.1;
            let user = entry.2;
            let event = GQLAlertEvent::generate_from_db_objects(alert, alert_source, user);
            alert_list.push(event);
        }
        return Ok(alert_list);
    }

    fn assigned_alerts(user_id: i32) -> FieldResult<GQLAlertList> {
        let query_response = controller::get_assigned_alerts(user_id);

        let mut alert_list: GQLAlertList = Vec::new();
        for entry in query_response.into_iter() {
            let alert = entry.0;
            let alert_source = entry.1;
            let user = entry.2;
            let event = GQLAlertEvent::generate_from_db_objects(alert, alert_source, user);
            alert_list.push(event);
        }
        return Ok(alert_list);
    }

    fn incident_reports() -> FieldResult<GQLIncidentReportList> {
        let query_response = controller::get_incident_reports();

        let mut incidents: HashMap<i32, GQLIncidentReport> = HashMap::new();

        let incident_reports: GQLIncidentReportList;
        for entry in query_response.into_iter() {
            let _incident_alert = entry.0;
            let incident_report = entry.1;
            let alert = entry.2 .0;
            let alert_source = entry.2 .1;
            let user = entry.2 .2;
            let event = GQLAlertEvent::generate_from_db_objects(alert, alert_source, user);

            if incidents.contains_key(&incident_report.id) {
                // assume incident related info is present. Just add alert
                incidents
                    .get_mut(&incident_report.id)
                    .unwrap()
                    .add_alert(event);
            } else {
                let incident_obj = GQLIncidentReport {
                    id: incident_report.id.to_string(),
                    segments_lost: incident_report.segments_lost.to_f64().unwrap_or(-1.0),
                    loss_details: incident_report.loss_details.to_string(),
                    cost: incident_report.cost.to_f64().unwrap_or(-1.0),
                    acked_at: incident_report.acked_at_str(),
                    resolved_at: incident_report.resolved_at_str().unwrap_or("".to_string()),
                    status: incident_report.status.to_string(),
                    resolution: incident_report.resolution.to_string(),
                    created_at: incident_report.created_at_str(),
                    last_updated: incident_report.last_updated_str(),
                    linked_alerts: Vec::from([event]),
                };
                incidents.insert(incident_report.id, incident_obj);
            }
        }
        incident_reports = incidents.into_values().collect();
        return Ok(incident_reports);
    }

    // fn subscriptions() -> FieldResult<GQLSubscription> {
    //     Ok()
    // }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_alert_source(mut alert_source: GQLNewAlertSource) -> FieldResult<bool> {
        // convert string params to json
        let parsed_params: jsonValue;
        match serde_json::from_str(alert_source.connection_params.as_str()) {
            Ok(params) => {
                parsed_params = params;
            }
            Err(e) => {
                log::error!(
                    "Cannot parse connection params as json. Error - {}",
                    e.to_string()
                );
                return Err(FieldError::new(e.to_string(), juniperValue::Null));
            }
        };

        let alert_source_obj = NewAlertSourceInfo {
            source_type: alert_source.source_type.as_str(),
            identifier: alert_source.identifier.as_str(),
            connection_params: &parsed_params,
            auth_type: alert_source.auth_type.as_str(),
            connect_url: alert_source.connect_url.as_str(),
            enabled: alert_source.enabled,
        };

        // Test connection to make sure data is valid

        match controller::create_alert_source(alert_source_obj) {
            Ok(res) => {
                return Ok(res);
            }
            Err(e) => {
                log::error!("{}", e.to_string());
                return Err(FieldError::new(e.to_string(), juniperValue::Null));
            }
        };
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
