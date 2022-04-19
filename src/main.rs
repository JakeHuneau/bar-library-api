use bar_library_api::configuration::get_configuration;
use bar_library_api::startup::run;
use bar_library_api::telemetry::{get_log_level, get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Set up tracing
    let subscriber = get_subscriber("barLibraryApi".into(), get_log_level(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to load configuration.");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());
    let listener = TcpListener::bind(format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    ))
    .expect("Failed to bind to port 8000");
    run(listener, connection_pool)?.await
}
