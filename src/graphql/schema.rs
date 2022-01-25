use super::controller;
use super::defs::{
    GQLAlertEvent, GQLAlertList, GQLIncidentReport, GQLIncidentReportList, GQLNewAlertEvent,
};
use bigdecimal::ToPrimitive;
use juniper::FieldResult;
use juniper::{EmptySubscription, RootNode};
use std::collections::HashMap;

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
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

        let alert_list: GQLIncidentReportList;
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
        alert_list = incidents.into_values().collect();
        return Ok(alert_list);
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_human(_alert: GQLNewAlertEvent) -> FieldResult<bool> {
        Ok(true)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
