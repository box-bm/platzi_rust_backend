extern crate diesel;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use std::env;
use tera::Tera;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use diesel::r2d2::Pool;
use diesel::r2d2::{self, ConnectionManager};

pub mod models;
pub mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use self::models::{NewPostHandler, Post};
use self::schema::posts::dsl::*;

#[get("/")]
async fn index(pool: web::Data<DbPool>, template_manager: web::Data<tera::Tera>) -> impl Responder {
    let mut conn = pool.get().expect("It doesn't connect to SQL");

    return match web::block(move || posts.load::<Post>(&mut conn)).await {
        Ok(data) => {
            let data = data.unwrap();
            // let data_ = [];

            // Enviamos, a través del contexto, los datos al HTML
            let mut ctx = tera::Context::new();
            ctx.insert("posts", &data);

            // Pasamos los datos al template index.html
            HttpResponse::Ok().content_type("text/html").body(
                template_manager.render("index.html", &ctx).unwrap()
            )
        }
        Err(err) => HttpResponse::Ok().body("hubo un error"),
    };
}

#[get("/blog/{blog_slug}")]
async fn blog(pool: web::Data<DbPool>, template_manager: web::Data<tera::Tera>, blog_slug: web::Path<String>) -> impl Responder {
    let mut conn = pool.get().expect("It doesn't connect to SQL");

    let url_slug = blog_slug.into_inner();

    return match web::block(move || posts.filter(slug.eq(url_slug)).load::<Post>(&mut conn)).await {
        Ok(data) => {
            let data = data.unwrap();
            
            if data.len() == 0 {
                return HttpResponse::NotFound().finish();
            }

            let data = &data[0];

            // Enviamos, a través del contexto, los datos al HTML
            let mut ctx = tera::Context::new();
            ctx.insert("post", &data);

            // Pasamos los datos al template index.html
            HttpResponse::Ok().content_type("text/html").body(
                template_manager.render("post.html", &ctx).unwrap()
            )
        }
        Err(err) => HttpResponse::Ok().body("hubo un error"),
    };
}

#[post["/new_post"]]
async fn insert(pool: web::Data<DbPool>, item: web::Json<NewPostHandler>) -> impl Responder {
    let mut conn = pool.get().expect("It doesn't connect to SQL");

    let new_post = NewPostHandler {
        title: item.title.clone(),
        body: item.body.clone(),
    };

    return match web::block(move || Post::create_post(&mut conn, &new_post)).await {
        Ok(data) => HttpResponse::Ok().body(format!("{:?}", data.ok())),
        Err(err) => HttpResponse::Ok().body("hubo un error"),
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DB url variable doesn't found");
    let port = env::var("PORT").expect("PORT variable doesn't found");
    let port: u16 = port.parse().unwrap();

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = Pool::builder()
        .build(manager)
        .expect("No se pudo construir la pool");

    HttpServer::new(move || {
        let tera_config =
            Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        App::new()
            .service(index)
            .service(insert)
            .service(blog)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera_config.clone()))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
