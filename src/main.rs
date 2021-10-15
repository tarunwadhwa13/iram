// #[macro_use]
// extern crate lazy_static;

use actix_web::{middleware::Logger, web, App, HttpRequest, HttpServer, Responder};
use log::info;
use log4rs;

use amp::configstore::CONFIG;

async fn manual_hello(_req: HttpRequest) -> impl Responder {
    format!("Hello There !!")
}

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
            .route("/health", web::get().to(manual_hello))
    })
    .bind(format!("{}:{}", CONFIG.server.host, CONFIG.server.port))?
    .run()
    .await
}
