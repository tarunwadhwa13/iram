use crate::alert_sources::get_alert_source_handler;
use actix_web::{web, HttpRequest, Responder, Result};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct WebhookResponse {
    pub status: bool,
    pub msg: String,
    pub error: Option<String>,
}

pub async fn handle_webhook(
    _req: HttpRequest,
    web::Path((source, identifier)): web::Path<(String, String)>,
) -> Result<impl Responder> {
    log::info!(
        "Creating alert source handler for - {} | Identifier - {}",
        source,
        identifier
    );
    let handler = get_alert_source_handler(
        source.to_lowercase().as_str(),
        identifier.to_lowercase().as_str(),
    );

    match handler {
        Err(e) => {
            return Ok(web::Json(WebhookResponse {
                status: false,
                msg: "Webhook not processed".to_string(),
                error: Some(e.to_string()),
            }))
        }
        Ok(obj) => match obj.process_webhook() {
            Err(e) => {
                return Ok(web::Json(WebhookResponse {
                    status: false,
                    msg: "Webhook not processed".to_string(),
                    error: Some(e.to_string()),
                }))
            }
            Ok(active_alerts) => {
                let insertion_response = obj.add_alert_to_db(active_alerts);

                match insertion_response {
                    Err(error) => {
                        return Ok(web::Json(WebhookResponse {
                            status: false,
                            msg: "Webhook processing failed. Failed to store alerts in DB"
                                .to_string(),
                            error: Some(error.to_string()),
                        }))
                    }
                    Ok(status) => {
                        if status == true {
                            return Ok(web::Json(WebhookResponse {
                                status: true,
                                msg: "Webhook processed successfully".to_string(),
                                error: None,
                            }));
                        } else {
                            return Ok(web::Json(WebhookResponse {
                                status: false,
                                msg: "Unexpected error occurred".to_string(),
                                error: None,
                            }));
                        }
                    }
                }
            }
        },
    };
}
