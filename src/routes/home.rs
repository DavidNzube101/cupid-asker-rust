use actix_web::{get, HttpResponse, Responder};
use tera::Tera;

#[get("/")]
pub async fn home_page(tera: actix_web::web::Data<Tera>) -> impl Responder {
    let rendered = tera.render("form.html", &tera::Context::new()).unwrap();
    HttpResponse::Ok().body(rendered)
}
