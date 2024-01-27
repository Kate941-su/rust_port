use actix_web::{Responder, HttpResponse};
use actix_web::web::Data;
use tera::Tera;

pub async fn index(tera: Data<Tera>) -> impl Responder {
    let mut context = tera::Context::new();
    context.insert("name", &"John Doe");

    let rendered = tera.render("index.html", &context);
    match rendered {
        //
        Ok(..) => HttpResponse::Ok().body(rendered.unwrap()),
        // TODO: Show the error message.
        Err(..) => panic!("Tera rendering could not execute.")
    }
    // HttpResponse::Ok().body(rendered.unwrap())
}

pub async fn not_found(tera: Data<Tera>) -> impl Responder {
    let mut context = tera::Context::new();
    let rendered = tera.render("404.html", &context);
    match rendered {
        Ok(..) => HttpResponse::Ok().body(rendered.unwrap()),
        // TODO: Show the error message.
        Err(..) => panic!("Tera rendering could not execute.")
    }
    // HttpResponse::Ok().body(rendered)
}