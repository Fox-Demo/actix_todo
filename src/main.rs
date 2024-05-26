use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Serialize;

mod api;
mod models;
mod repo;

#[derive(Serialize)]
struct Response {
    message: String,
}

#[get("/health")]
async fn health_checker() -> impl Responder {
    HttpResponse::Ok().json(Response {
        message: "I am healthy".to_string(),
    })
}

async fn not_found() -> Result<HttpResponse> {
    Ok(HttpResponse::NotFound().json(Response {
        message: "Not Found".to_string(),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let todo_db = repo::database::Database::new();
    let app_data = web::Data::new(todo_db);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(api::api::config)
            .service(health_checker)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
