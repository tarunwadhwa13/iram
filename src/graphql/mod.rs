mod controller;
mod defs;
pub mod schema;

use actix_web::Responder;
use crate::settings;

use actix_web::{web, web::Data, HttpResponse};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use schema::{create_schema, Schema};
use std::sync::Arc;

pub fn register_graphql_service(cfg: &mut web::ServiceConfig) {
    let graphql_schema = std::sync::Arc::new(create_schema());
    cfg.app_data(Data::new(graphql_schema.clone()));
    cfg.service(web::resource("/graphql").route(web::post().to(graphql)));
    cfg.service(web::resource("/graphiql").route(web::get().to(graphiql)));
}

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    _config: web::Data<settings::Settings>,
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    let user = data.execute(&st, &()).await;
    HttpResponse::Ok().json(user)
}
