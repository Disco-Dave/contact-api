mod contact;
mod health_check;

use actix_web::web;

pub fn configure(config: &mut web::ServiceConfig) {
    config
        .route("/", web::post().to(contact::handler))
        .route("/health-check", web::get().to(health_check::handler));
}
