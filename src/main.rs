use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::config::get_config;
use zero2prod::email_client::EmailClient;
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
        .connect_lazy_with(config.database.with_db());

    let sender_email = config.email_client.sender().expect("Invalid sender email address.");

    let email_client = EmailClient::new(
        config.email_client.base_url,
        config.email_client.auth_token,
        sender_email,
    );

    run(listener, connection_pool, email_client)?.await
}
