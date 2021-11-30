// #[macro_use]
// extern crate lazy_static;

use actix_web::{middleware::Logger, web, App, Error, HttpResponse, HttpServer};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use log::info;
use log4rs;
use std::sync::Arc;

use amp::configstore::CONFIG;
use amp::graphql::schema::{create_schema, Schema};

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute_sync(&st, &());
        serde_json::to_string(&res)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Booting Up Systems 🚀");
    log4rs::init_file("config/log_config.yml", Default::default()).unwrap();

    let graphql_schema = std::sync::Arc::new(create_schema());

    println!(
        "✨ Starting Alert Monitoring Platform ✨ !!

         █████╗  ███╗   ███╗ ██████╗ 
        ██╔══██╗ ████╗ ████║ ██╔══██╗
        ███████║ ██╔████╔██║ ██████╔╝
        ██╔══██║ ██║╚██╔╝██║ ██╔═══╝ 
        ██║  ██║ ██║ ╚═╝ ██║ ██║     
        ╚═╝  ╚═╝ ╚═╝     ╚═╝ ╚═╝              
    "
    );

    info!(
        "Starting Webserver at {}:{} | Env - {}",
        CONFIG.server.host, CONFIG.server.port, CONFIG.env
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(CONFIG.clone())
            .data(graphql_schema.clone())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind(format!("{}:{}", CONFIG.server.host, CONFIG.server.port))?
    .run()
    .await
}
