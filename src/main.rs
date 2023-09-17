use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
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
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(configuration.database.connection_string().expose_secret())
        .expect("Failed to connect to Postgres.");

    let listener = std::net::TcpListener::bind(format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    ))?;
    run(listener, connection_pool)?.await
}
