use actix_files as fs;
use actix_web::{web, App, HttpServer, Responder, get, post, HttpResponse};
use tera::Tera;

struct AppState {
    app_name: String,
}

async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut context = tera::Context::new();
    context.insert("name", &"John Doe");

    let rendered = tera.render("index.html", &context).expect("Error rendering template");

    HttpResponse::Ok().body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("template/**/*").expect("Error parsing templates");
    HttpServer::new(move || {
        App::new().app_data(web::Data::new(tera.clone()))
            // fs::Files::new("{mount_point}", "{relative directory}")
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .route("/", web::get().to(index))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}