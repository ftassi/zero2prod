use actix_web::{post, web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[post("/subscriptions")]
pub async fn subscriptions(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
}
