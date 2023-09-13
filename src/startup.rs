use crate::routes::health_check;
use crate::routes::subscriptions;
use actix_web::{dev::Server, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check).service(subscriptions))
        .listen(listener)?
        .run();

    Ok(server)
}
