use tracing::{subscriber::set_global_default, Subscriber};
use tracing_appender::{self, non_blocking, rolling};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

use super::settings::LogSettings;

pub fn get_subscriber(
    settings: &LogSettings,
) -> (impl Subscriber + Send + Sync, non_blocking::WorkerGuard) {
    let file_appender = rolling::daily(&settings.log_dir, "contact-api.log");
    let (non_blocking, guard) = non_blocking(file_appender);

    let subscriber = Registry::default()
        .with(EnvFilter::new(&settings.directive))
        .with(fmt::layer().with_ansi(false).with_writer(non_blocking))
        .with(fmt::layer().pretty());

    (subscriber, guard)
}

pub fn init(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
