use secrecy::ExposeSecret;
use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subcriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    init_subscriber(get_subcriber(
        "zero2prod".into(),
        "info".into(),
        std::io::stdout,
    ));
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool =
        PgPool::connect(&configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres.");

    let listener =
        std::net::TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))?;
    run(listener, connection_pool)?.await
}
