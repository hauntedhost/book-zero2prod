use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::config::get_config;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    let config = get_config().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", config.app_port);

    let listener = TcpListener::bind(address).expect("Failed to bind to port");
    let connection_pool = PgPool::connect(&config.database.connect_string())
        .await
        .expect("Failed to connect to Postgres");

    run(listener, connection_pool)?.await
}
