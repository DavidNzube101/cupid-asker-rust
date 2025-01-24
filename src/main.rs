use actix_web::{web, App, HttpServer};
use tera::Tera;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("src/templates/**/*").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .service(routes::home::home_page)
            .service(routes::form_handler::submit_form)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
