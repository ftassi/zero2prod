use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};

use reqwest::StatusCode;

#[tokio::test]
async fn health_check_should_return_ok() {
    let app_data = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", &app_data.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(Some(0), response.content_length());
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let connection_string = config.connection_string();
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres.");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database.");

    connection_pool
}

struct AppData {
    address: String,
    connection: PgPool,
}

async fn spawn_app() -> AppData {
    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let connection = configure_database(&configuration.database).await;
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server =
        zero2prod::startup::run(listener, connection.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    AppData {
        address: format!("http://127.0.0.1:{}", port),
        connection,
    }
}
