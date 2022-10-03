#[macro_use]
extern crate diesel;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;

#[get("/")]
async fn hello_world() -> impl Responder {
    format!("Hello world!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello_world))
        .bind(("localhost", 8080))?
        .run()
        .await
}
