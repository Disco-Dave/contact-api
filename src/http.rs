mod routes;

use actix_web::{dev::Server, web};
use actix_web::{App, HttpServer};
use tracing_actix_web::TracingLogger;

use super::email::EmailService;
use super::settings::HttpSettings;

pub struct HttpApp {
    pub server: Server,
    pub port: u16,
}

pub fn start(settings: HttpSettings, email_service: EmailService) -> std::io::Result<HttpApp> {
    let listener = settings.tcp_listener()?;

    let port = listener.local_addr()?.port();

    let email_service = web::Data::new(email_service);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .configure(routes::configure)
            .app_data(email_service.clone())
    })
    .listen(listener)?
    .run();

    Ok(HttpApp { server, port })
}
