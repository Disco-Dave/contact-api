use contact_api::settings::EmailSettings;
use contact_api::settings::Settings;

pub struct TestApp {
    pub address: String,
    pub email_settings: EmailSettings,
}

pub async fn spawn_app() -> TestApp {
    let settings = {
        let mut settings = Settings::new().expect("Unable to read settings.");
        settings.http.port = 0;
        settings.email.from = format!("{}@test.fake", uuid::Uuid::new_v4());
        settings
    };
    let host = settings.http.host.clone();

    let app =
        contact_api::start(settings.http, settings.email.clone()).expect("Unable to start app");
    let address = format!("http://{}:{}", host, app.port);

    tokio::spawn(app.server);

    TestApp {
        address,
        email_settings: settings.email,
    }
}
