use actix_web::{http::header::ContentType, web, HttpResponse, Responder};
use tera::Tera;

pub async fn index(tera: web::Data<Tera>) -> impl Responder {
    //render the template
    let rendered = match tera.render("home/index.html", &tera::Context::new()) {
        Ok(t) => t,
        Err(e) => {
            println!("Error while loading template in handler: {}", e);
            std::process::exit(1);
        }
    };
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered)
}

pub async fn privacy(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
