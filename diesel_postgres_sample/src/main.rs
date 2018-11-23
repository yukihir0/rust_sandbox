#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

use self::diesel::prelude::*;
use self::models::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    use schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("id:{}, name:{}", user.id, user.name);
    }
}
