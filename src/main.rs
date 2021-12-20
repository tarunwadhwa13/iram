// #[macro_use]
// extern crate lazy_static;

use actix_web::{middleware::Logger, web, App, HttpServer, http};
use actix_cors::Cors;
use log::info;
use log4rs;

use iram::configstore::CONFIG;
use iram::graphql::register_graphql_service;
use iram::orchestrator::handle_webhook;
use iram::db;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Booting Up Systems 🚀");
    log4rs::init_file("config/log_config.yml", Default::default()).unwrap();

    println!("✅ Checking connection to downstream services ✅");
    let _ = db::get_connection();

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
        let cors = Cors::default()
              .allow_any_origin()
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .data(CONFIG.clone())
            .service(web::resource("/api/webhook/{source}/{identifier}").route(web::get().to(handle_webhook)))
            .configure(register_graphql_service)
    })
    .bind(format!("{}:{}", CONFIG.server.host, CONFIG.server.port))?
    .run()
    .await
}
