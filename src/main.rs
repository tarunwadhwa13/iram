// #[macro_use]
// extern crate lazy_static;

use actix_web::{middleware::Logger, web, App, HttpServer};
use log::info;
use log4rs;

use amp::configstore::CONFIG;
use amp::graphql::register_graphql_service;
use amp::orchestrator::handle_webhook;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Booting Up Systems 🚀");
    log4rs::init_file("config/log_config.yml", Default::default()).unwrap();

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
            .service(web::resource("/api/webhook/{source}/{identifier}").route(web::get().to(handle_webhook)))
            .configure(register_graphql_service)
    })
    .bind(format!("{}:{}", CONFIG.server.host, CONFIG.server.port))?
    .run()
    .await
}
