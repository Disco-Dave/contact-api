use contact_api::logging;
use contact_api::settings::Settings;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::new().expect("Failed to get application settings.");

    let (subscriber, _guard) = logging::get_subscriber(&settings.log);
    logging::init(subscriber);

    let app = contact_api::start(settings.http, settings.email)?;

    app.server.await
}
