use juniper::{GraphQLObject, GraphQLInputObject};

// Here, struct names can be same as db Models. Hence to remove possibility of duplicacy, we add GQL (short for Graphql) as prefix for all structs

#[derive(GraphQLObject)]
pub struct GQLUser {
    pub id: i32,
    pub username: String,
    pub is_active: bool,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub is_admin: bool,
    pub last_login: String,
    pub date_joined: String
}

#[derive(GraphQLObject)]
pub struct GQLAlertEvent {
    pub id: String,
    pub source_type: String,
    pub source: String,
    pub created_at: String,
    pub last_updated: String,
    pub age: String,
    pub entity: String,
    pub subject: String,
    pub priority: String,
    pub state: String,
    pub description: String,
    pub assigned_to: GQLUser,
}

pub type GQLAlertList = Vec<GQLAlertEvent>;

#[derive(GraphQLObject)]
pub struct GQLIncidentReport {
    pub id: String,
    pub segments_lost: f64,
    pub loss_details: String,
    pub cost: f64,
    pub acked_at: String,
    pub resolved_at: String,
    pub status: String,
    pub resolution: String,
    pub created_at: String,
    pub last_updated: String,
    pub linked_alerts: Vec<GQLAlertEvent>
}

impl GQLIncidentReport {
    pub fn add_alert(&mut self, event: GQLAlertEvent) {
        self.linked_alerts.push(event)
    }
}

pub type GQLIncidentReportList = Vec<GQLIncidentReport>;

#[derive(GraphQLInputObject)]
#[graphql(description = "An alert event which needs attention")]
pub struct GQLNewAlertEvent {
    pub source: String,
    pub created_at: String,
    pub age: String,
    pub entity: String,
}

