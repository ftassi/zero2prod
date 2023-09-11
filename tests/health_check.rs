use reqwest::StatusCode;

#[tokio::test]
async fn health_check_should_return_ok() {
    spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
        
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
