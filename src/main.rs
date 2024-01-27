use actix_files as fs;
use actix_web::{web, App, HttpServer, Responder, get, post, HttpResponse};
use actix_web::web::resource;
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
        App::new()
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(
                // warning: If implementing web::scope() as root directory => web::scope("/")
                // you can only access as following path.
                // .route("skills", web::get().to(view::general::skills))
                // http://{hostname}// (<= unexpected slash) skills
                web::scope("/app")
            //         .route("", web::get().to(view::general::index))
            //         .route("introduce", web::get().to(view::general::introduce))
            //         .route("products", web::get().to(view::general::products))
            //         .route("skills", web::get().to(view::general::skills))
            .service(view::general::index)
            .service(view::general::introduce)
            .service(view::general::products)
            .service(view::general::skills)
            )
            .app_data(web::Data::new(tera.clone()))
            .default_service(web::route().to(view::general::not_found))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}