use crate::diesel::QueryDsl;
use juniper::graphql_value;
use juniper::{EmptySubscription, RootNode};
use juniper::{FieldError, FieldResult};

use crate::db::get_connection;
use juniper::{GraphQLInputObject, GraphQLObject};

use crate::errors::GenericAlertSourceError;
use crate::models::{AlertSourceInfo, Alerts, Users};
use crate::schema::{alert_source_info, alerts, users};
use diesel::{ExpressionMethods, RunQueryDsl};

#[derive(GraphQLObject)]
struct AlertEvent {
    id: String,
    source_type: String,
    source: String,
    created_at: String,
    last_updated: String,
    age: String,
    entity: String,
    subject: String,
    priority: String,
    state: String,
    description: String,
    assigned_to: String,
}

type AlertList = Vec<AlertEvent>;

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn active_alerts() -> FieldResult<AlertList> {
        let connection = get_connection().unwrap();

        let query_response = alerts::dsl::alerts
            .inner_join(alert_source_info::table)
            .inner_join(users::table)
            .filter(alert_source_info::dsl::enabled.eq(true))
            .filter(alerts::dsl::state.ne("resolved".to_string()))
            .load::<(Alerts, AlertSourceInfo, Users)>(&connection)
            .expect("Encountered DB Error while loading alerts assigned to user");

        log::info!("Got {} active alerts", query_response.len());

        let mut alert_list: AlertList = Vec::new();
        for entry in query_response.into_iter() {
            let alert = entry.0;
            let alert_source = entry.1;
            let user = entry.2;
            let event = AlertEvent {
                id: alert.id.to_string(),
                source_type: alert_source.source_type,
                source: alert_source.identifier,
                created_at: alert.created_at.to_string(),
                last_updated: alert.last_updated.to_string(),
                age: (chrono::Local::now().signed_duration_since(alert.created_at)).to_string(),
                entity: alert.entity,
                subject: alert.subject,
                state: alert.state,
                priority: alert.priority,
                description: alert.description,
                assigned_to: user.username,
            };
            alert_list.push(event);
        }
        return Ok(alert_list);
    }

    fn past_alerts(_time_limit: i32) -> FieldResult<AlertList> {
        let connection = get_connection().unwrap();

        let query_response = alerts::dsl::alerts
            .inner_join(alert_source_info::table)
            .inner_join(users::table)
            .filter(alert_source_info::dsl::enabled.eq(true))
            .filter(alerts::dsl::state.eq("resolved".to_string()))
            .load::<(Alerts, AlertSourceInfo, Users)>(&connection)
            .expect("Encountered DB Error while loading alerts assigned to user");

        log::info!("Got {} past alerts", query_response.len());

        let mut alert_list: AlertList = Vec::new();
        for entry in query_response.into_iter() {
            let alert = entry.0;
            let alert_source = entry.1;
            let user = entry.2;
            let event = AlertEvent {
                id: alert.id.to_string(),
                source_type: alert_source.source_type,
                source: alert_source.identifier,
                created_at: alert.created_at.to_string(),
                last_updated: alert.last_updated.to_string(),
                age: (chrono::Local::now().signed_duration_since(alert.created_at)).to_string(),
                entity: alert.entity,
                subject: alert.subject,
                state: alert.state,
                priority: alert.priority,
                description: alert.description,
                assigned_to: user.username,
            };
            alert_list.push(event);
        }
        return Ok(alert_list);
    }

    fn assigned_alerts(user_id: i32) -> FieldResult<AlertList> {
        let connection = get_connection().unwrap();

        let query_response = alerts::dsl::alerts
            .inner_join(alert_source_info::table)
            .inner_join(users::table)
            .filter(alert_source_info::dsl::enabled.eq(true))
            .filter(alerts::dsl::state.ne("resolved".to_string()))
            .filter(alerts::dsl::assigned_user_id.eq(user_id))
            .load::<(Alerts, AlertSourceInfo, Users)>(&connection)
            .expect("Encountered DB Error while loading alerts assigned to user");

        log::info!(
            "Got {} active alerts assigned to user - {}",
            query_response.len(),
            user_id
        );

        if query_response.len() == 0 {
            let err_msg = "No active alert assigned to requested user found in database";
            return Err(FieldError::new(
                GenericAlertSourceError(err_msg.to_string()),
                graphql_value!({ "internal_error": ""}),
            ));
        } else {
            let mut alert_list: AlertList = Vec::new();
            for entry in query_response.into_iter() {
                let alert = entry.0;
                let alert_source = entry.1;
                let user = entry.2;
                let event = AlertEvent {
                    id: alert.id.to_string(),
                    source_type: alert_source.source_type,
                    source: alert_source.identifier,
                    created_at: alert.created_at.to_string(),
                    last_updated: alert.last_updated.to_string(),
                    age: (chrono::Local::now().signed_duration_since(alert.created_at)).to_string(),
                    entity: alert.entity,
                    subject: alert.subject,
                    state: alert.state,
                    priority: alert.priority,
                    description: alert.description,
                    assigned_to: user.username,
                };
                alert_list.push(event);
            }
            return Ok(alert_list);
        }
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "An alert event which needs attention")]
struct NewAlertEvent {
    source: String,
    created_at: String,
    age: String,
    entity: String,
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_human(_alert: NewAlertEvent) -> FieldResult<bool> {
        Ok(true)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
