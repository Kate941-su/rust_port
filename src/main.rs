use actix_files as fs;
use actix_web::{web, App, HttpServer, Responder, get, post, HttpResponse};
use tera::Tera;

// Self modules
mod view;

struct AppState {
    app_name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new("template/**/*").expect("Error parsing templates");
        // fs::Files::new("{mount_point}", "{relative directory}")
        App::new().app_data(web::Data::new(tera.clone()))
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(web::scope("/")
                .route("", web::get().to(view::general::index))
            )
            .default_service(web::route().to(view::general::not_found))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}