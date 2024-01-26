use actix_web::{web, App, HttpServer, Responder, get, post};

struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}



async fn index_test() -> impl Responder {
    "Hello world! I'm Kaito!!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // App::new().service(
        //     // prefixes all resources and routes attached to it...
        //     web::scope("/app")
        //         // ...so this handles requests for `GET /app/index.html`
        //         .route("/index.html", web::get().to(index)),
        // )

        // You will be able to see Hello Actix Web!
        App::new().app_data(web::Data::new(AppState{
            app_name: String::from("Actix Web"),
        })).service(index)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}