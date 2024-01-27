use actix_web::{Responder, HttpResponse,get};
use actix_web::web::Data;
use tera::Tera;

#[get("")]
pub async fn index(tera: Data<Tera>) -> impl Responder {
    let mut context = tera::Context::new();
    context.insert("name", &"John Doe");
    println!("through index");
    let rendered = tera.render("index.html", &context);
    match rendered {
        Ok(..) => HttpResponse::Ok().body(rendered.unwrap()),
        // TODO: Show the error message.
        Err(..) => panic!("Tera rendering could not execute.")
    }
    // HttpResponse::Ok().body(rendered.unwrap())
}

#[get("introduce")]
pub async fn introduce(tera: Data<Tera>) -> impl Responder {
    let mut context = tera::Context::new();
    println!("through");
    let rendered = tera.render("introduce.html", &context);
    match rendered {
        Ok(..) => HttpResponse::Ok().body(rendered.unwrap()),
        // TODO: Show the error message.
        Err(..) => panic!("Tera rendering could not execute.")
    }
    // HttpResponse::Ok().body(rendered.unwrap())
}
#[get("products")]
pub async fn products(tera: Data<Tera>) -> impl Responder {
    let mut context = tera::Context::new();
    let rendered = tera.render("products.html", &context);
    match rendered {
        Ok(..) => HttpResponse::Ok().body(rendered.unwrap()),
        // TODO: Show the error message.
        Err(..) => panic!("Tera rendering could not execute.")
    }
    // HttpResponse::Ok().body(rendered.unwrap())
}

#[get("skills")]
pub async fn skills(tera: Data<Tera>) -> impl Responder {
    let mut context = tera::Context::new();
    let rendered = tera.render("skills.html", &context);
    match rendered {
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
        Ok(..) => HttpResponse::NotFound().body(rendered.unwrap()),
        // TODO: Show the error message.
        Err(..) => panic!("Tera rendering could not execute.")
    }
    // HttpResponse::Ok().body(rendered)
}