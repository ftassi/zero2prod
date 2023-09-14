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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app_data = spawn_app().await;
    let configuration = get_configuration().expect("Failed to read configuration.");

    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_data.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execut request");

    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app_data.connection)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app_data = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%guin", "missing the email"),
        ("email=ursula_le_guid%40gmail.com", "missing the name"),
        ("", "missing both"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_data.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execut request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let connection_string = config.connection_string();
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres.");

    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
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
