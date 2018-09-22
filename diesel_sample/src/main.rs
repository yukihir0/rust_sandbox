#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::env::args;
use std::io::{stdin, Read};

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn show_post() {
    use models::{Post};
    use schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}

pub fn create_post(conn: &SqliteConnection, title: &str, body: &str) -> usize {
    use models::{NewPost};
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post")
}

fn write_post() {
    let connection = establish_connection();

    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character
    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        title, EOF
    );
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let _ = create_post(&connection, title, &body);
    println!("\nSaved draft {}", title);
}

fn publish_post() {
    use schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    
    let connection = establish_connection();

    let _ = diesel::update(posts.find(id))
        .set(published.eq(true))
        .execute(&connection)
        .expect(&format!("Unable to find post {}", id));

    let post: models::Post = posts
        .find(id)
        .first(&connection)
        .expect(&format!("Unable to find post {}", id));

    println!("Published post {}", post.title);
}

fn delete_post() {
    use schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}

fn main() {
    show_post();
    write_post();
    publish_post(); // cargo run <id>
    delete_post();  // cargo run <title>
}
