extern crate diesel;

use dotenvy::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

pub mod models;
pub mod schema;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DB url variable doesn't found");

    let mut conn =
        PgConnection::establish(&database_url).expect("we didn't connect to the database");

    // use self::models::{NewPost, Post};
    use self::models::{Post};
    use self::schema::posts::dsl::*;
    // use crate::schema::posts;

    /*
    // Create new Register


    let new_post = NewPost {
        title: "Mi first blog",
        body: "Lorem",
        slug: "first-post",
    };

    let new_post_2 = NewPost {
        title: "Mi second blog",
        body: "Lorem",
        slug: "second-post",
    };

    let new_post_3 = NewPost {
        title: "My third post",
        body: "Lorem",
        slug: "thirth-post",
    };

    let post: Post = diesel::insert_into(posts::table)
        .values(&vec![new_post, new_post_2, new_post_3])
        .get_result(&mut conn)
        .expect("Error when insert");

    */

    /*
    // Update post
    let post_update = diesel::update(posts.filter(id.eq(3)))
        .set((slug.eq("third-blogpost"), title.eq("Mi tercer blog")))
        .get_result::<Post>(&mut conn)
        .expect("Error updating record");

    println!("{:?}", post_update);
    */

    /*
    // Delete post
    diesel::delete(posts.filter(slug.like("%-post%")))
    .execute(&mut conn)
    .expect("Have problems when delete register");
    */

    /*
    Select post
    we can use:
    Where: with .filter() (into filter use eq or the condition to filter into another ())
    limit: limit(*number*)
    see the documentation
     */
    let posts_result = posts
        .load::<Post>(&mut conn)
        .expect("Error excecuting query");

    for post in posts_result {
        println!("{:?}", post);
    }
}
