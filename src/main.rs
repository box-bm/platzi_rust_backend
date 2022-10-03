extern crate diesel;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use std::env;
use std::fmt::format;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use diesel::r2d2::Pool;
use diesel::r2d2::{self, ConnectionManager};

pub mod models;
pub mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use self::models::Post;
use self::schema::posts;
use self::schema::posts::dsl::*;

#[get("/")]
async fn index(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("It doesn't connect to SQL");

    return match web::block(move || posts.load::<Post>(&mut conn)).await {
        Ok(data) => {
            HttpResponse::Ok().body(format!("{:?}", data.ok()))
        }
        Err(err) => HttpResponse::Ok().body("hubo un error"),
    };
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DB url variable doesn't found");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = Pool::builder()
        .build(manager)
        .expect("No se pudo construir la pool");

    HttpServer::new(move || {
        App::new()
            .service(index)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
