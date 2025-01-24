use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CardRequest {
    your_name: String,
    recipient_name: String,
    message: String,
}

#[post("/submit")]
pub async fn submit_form(form: web::Form<CardRequest>) -> impl Responder {
    let response = format!(
        "Thank you, {}! Your Valentine's card for {} has been received with the message: '{}'.",
        form.your_name, form.recipient_name, form.message
    );
    HttpResponse::Ok().body(response)
}
