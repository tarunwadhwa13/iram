use super::defs::{GQLAlertEvent, GQLUser};
use crate::db::get_connection;
use crate::diesel::QueryDsl;
use crate::errors::GenericAlertSourceError;
use crate::models::{
    AlertSourceInfo, Alerts, IncidentAlert, IncidentReport, NewAlertSourceInfo, Users,
};
use crate::schema::{alert_source_info, alerts, incident_alert, incident_report, users};
use diesel::{ExpressionMethods, RunQueryDsl};

pub fn get_active_alerts() -> Vec<(Alerts, AlertSourceInfo, Users)> {
    let connection = &mut get_connection().unwrap();

    let query_response = alerts::dsl::alerts
        .inner_join(alert_source_info::table)
        .inner_join(users::table)
        .filter(alert_source_info::dsl::enabled.eq(true))
        .filter(alerts::dsl::state.ne("resolved".to_string()))
        .load::<(Alerts, AlertSourceInfo, Users)>(connection)
        .expect("Encountered DB Error while loading alerts assigned to user");

    log::info!("Got {} active alerts", query_response.len());
    return query_response;
}

pub fn get_past_alerts() -> Vec<(Alerts, AlertSourceInfo, Users)> {
    let connection = &mut get_connection().unwrap();

    let query_response = alerts::dsl::alerts
        .inner_join(alert_source_info::table)
        .inner_join(users::table)
        .filter(alert_source_info::dsl::enabled.eq(true))
        .filter(alerts::dsl::state.eq("resolved".to_string()))
        .load::<(Alerts, AlertSourceInfo, Users)>(connection)
        .expect("Encountered DB Error while loading alerts assigned to user");

    log::info!("Got {} past alerts", query_response.len());
    return query_response;
}

pub fn get_assigned_alerts(user_id: i32) -> Vec<(Alerts, AlertSourceInfo, Users)> {
    let connection = &mut get_connection().unwrap();

    let query_response = alerts::dsl::alerts
        .inner_join(alert_source_info::table)
        .inner_join(users::table)
        .filter(alert_source_info::dsl::enabled.eq(true))
        .filter(alerts::dsl::state.ne("resolved".to_string()))
        .filter(alerts::dsl::assigned_user_id.eq(user_id))
        .load::<(Alerts, AlertSourceInfo, Users)>(connection)
        .expect("Encountered DB Error while loading alerts assigned to user");

    log::info!(
        "Got {} active alerts assigned to user - {}",
        query_response.len(),
        user_id
    );
    return query_response;
}

// pub fn get_incident_reports() -> Vec<(
//     IncidentAlert,
//     IncidentReport,
//     (Alerts, AlertSourceInfo, Users),
// )> {
//     let connection = get_connection().unwrap();

//     let query_response = incident_alert::dsl::incident_alert
//         .inner_join(incident_report::table)
//         .inner_join(
//             alerts::table
//                 .inner_join(alert_source_info::table)
//                 .inner_join(users::table),
//         )
//         .load::<(
//             IncidentAlert,
//             IncidentReport,
//             (Alerts, AlertSourceInfo, Users),
//         )>(&mut connection)
//         .expect("Encountered DB Error while loading alerts assigned to user");

//     log::info!("Got {} incident reports", query_response.len());
//     return query_response;
// }

pub fn get_alert_sources() -> Vec<AlertSourceInfo> {
    let connection = &mut get_connection().unwrap();

    let query_response = alert_source_info::dsl::alert_source_info
        .load::<AlertSourceInfo>(connection)
        .expect("Encountered DB Error while fetching alert sources");

    log::info!("Got {} alert sources registered", query_response.len());
    return query_response;
}

pub fn create_alert_source(
    alert_source: NewAlertSourceInfo,
) -> Result<bool, GenericAlertSourceError> {
    let connection = &mut get_connection().unwrap();

    let rows_inserted = diesel::insert_into(alert_source_info::table)
        .values(&alert_source)
        .execute(connection);

    if let Ok(i) = rows_inserted {
        return Ok(true);
    } else {
        return Err(GenericAlertSourceError(
            "Failed to insert entry in Database".to_string(),
        ));
    }
}

impl GQLAlertEvent {
    pub fn generate_from_db_objects(
        alert: Alerts,
        alert_source: AlertSourceInfo,
        user: Users,
    ) -> Self {
        Self {
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
            assigned_to: GQLUser {
                id: user.id,
                username: user.username.to_string(),
                is_active: user.is_active,
                first_name: user.first_name.to_string(),
                last_name: user.last_name.to_string(),
                email: user.email.to_string(),
                is_admin: user.is_admin,
                last_login: user.last_login_str().unwrap_or("".to_string()),
                date_joined: user.date_joined_str().unwrap_or("".to_string()),
            },
        }
    }
}
