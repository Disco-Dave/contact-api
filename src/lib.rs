mod domain;
mod email;
mod http;
pub mod logging;
pub mod settings;

pub use http::HttpApp;

use settings::{EmailSettings, HttpSettings};

pub fn start(
    http_settings: HttpSettings,
    email_settings: EmailSettings,
) -> std::io::Result<HttpApp> {
    let email_service = email::EmailService::new(email_settings);

    http::start(http_settings, email_service)
}
