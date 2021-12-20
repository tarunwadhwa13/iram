use juniper::FieldResult;
use juniper::{EmptySubscription, RootNode};


use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "An alert event which needs attention")]
struct AlertEvent {
    id: String,
    source: String,
    created_at: String,
    last_updated: String,
    age: String,
    entity: String,
    subject: String,
    priority: String,
    status: String
}

type AlertList = Vec<AlertEvent>;

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn active_alerts() -> FieldResult<AlertList> {
        let alert = AlertEvent {
            id: "1234".to_owned(),
            source: "Luke".to_owned(),
            created_at: "2020-10-10 02:30:04".to_owned(),
            last_updated: "2020-10-10 02:30:04".to_owned(),
            age: "2m".to_owned(),
            entity: "Unknown".to_owned(),
            subject: "Unknown".to_owned(),
            status: "New".to_owned(),
            priority: "P2".to_owned()
        };
        let mut alert_list: AlertList = Vec::new();
        alert_list.push(alert);
        return Ok(alert_list);
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "An alert event which needs attention")]
struct NewAlertEvent {
    source: String,
    created_at: String,
    age: String,
    entity: String
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_human(alert: NewAlertEvent) -> FieldResult<AlertEvent> {
        Ok(AlertEvent {
            id: "1234".to_owned(),
            source: alert.source,
            created_at: "2020-10-10 02:30:04".to_owned(),
            last_updated: "2020-10-10 02:30:04".to_owned(),
            age: "2m".to_owned(),
            entity: "Unknown".to_owned(),
            status: "New".to_owned(),
            subject: "Unknown".to_owned(),
            priority: "P2".to_owned()
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}