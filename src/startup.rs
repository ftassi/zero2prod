use crate::routes::health_check;
use crate::routes::subscriptions;
use actix_web::web;
use actix_web::{dev::Server, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .service(health_check)
            .service(subscriptions)
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
