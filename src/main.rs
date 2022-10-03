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

use self::models::{Post, NewPost, NewPostHandler};
use self::schema::posts::dsl::*;

#[get("/")]
async fn index(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("It doesn't connect to SQL");

    return match web::block(move || posts.load::<Post>(&mut conn)).await {
        Ok(data) => HttpResponse::Ok().body(format!("{:?}", data.ok())),
        Err(err) => HttpResponse::Ok().body("hubo un error"),
    };
}

#[post["/new_post"]]
async fn insert(pool: web::Data<DbPool>, item: web::Json<NewPostHandler>) -> impl Responder {
    let mut conn = pool.get().expect("It doesn't connect to SQL");

    let new_post = NewPost {
        title: item.title.clone().as_str(),
        slug: "fouth-post",
        body: item.body.as_str()
    };

    return match web::block(move || {Post::create_post(conn, new_post)}).await {
        Ok(data) => HttpResponse::Ok().body(format!("{:?}", data.ok())),
        Err(err) => HttpResponse::Ok().body("hubo un error"),
    };
}

#[actix_web::main]
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
            .service(insert)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
