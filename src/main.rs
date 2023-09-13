use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let listener =
        std::net::TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))?;
    run(listener)?.await
}
