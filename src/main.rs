use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::config::get_config;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_config().expect("Failed to read configuration");
    let address = format!("{}:{}", config.app.host, config.app.port);
    let listener = TcpListener::bind(address).expect("Failed to bind to port");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(&config.database.connect_string().expose_secret())
        .expect("Failed to connect to Postgres");

    run(listener, connection_pool)?.await
}
