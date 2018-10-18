#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use models::{User, NewUser, Post, NewPost, Tag, NewTag, PostTag, NewPostTag};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn initialize_database(conn: &SqliteConnection) {
    use schema::users::dsl::*;
    use schema::posts::dsl::*;
    use schema::tags::dsl::*;
    use schema::posts_tags::dsl::*;

    diesel::delete(users)
        .execute(conn)
        .expect("Error delete users");

    diesel::delete(posts)
        .execute(conn)
        .expect("Error delete posts");

    diesel::delete(tags)
        .execute(conn)
        .expect("Error delete tags");

    diesel::delete(posts_tags)
        .execute(conn)
        .expect("Error delete posts_tags");

    // User
    create_user(conn, "hoge");
    create_user(conn, "fuga");

    // Post
    create_post(conn, "post01 by hoge", 1);
    create_post(conn, "post02 by hoge", 1);
    create_post(conn, "post03 by fuga", 2);
    create_post(conn, "post04 by fuga", 2);

    // Tag
    create_tag(conn, "tag01");
    create_tag(conn, "tag02");
    create_tag(conn, "tag03");

    // PostTag
    create_post_tag(conn, 1, 1);
    create_post_tag(conn, 2, 1);
    create_post_tag(conn, 2, 2);
    create_post_tag(conn, 3, 3);
    create_post_tag(conn, 4, 2);
    create_post_tag(conn, 4, 3);
}

pub fn select_user(conn: &SqliteConnection, user_id: i32) -> User {
    use schema::users::dsl::*;

    users
        .find(user_id)
        .first(conn)
        .expect("Error select user")
}

pub fn create_user<S: Into<String>>(conn: &SqliteConnection, name: S) {
    use schema::users;

    let new_user = NewUser {
        name: name.into(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error create user");
}

pub fn delete_user(conn: &SqliteConnection, user_id: i32) {
    use schema::users::dsl::*;

    diesel::delete(
        users.find(user_id) 
    )
        .execute(conn)
        .expect("Error delete user");
}

pub fn select_post(conn: &SqliteConnection, post_id: i32) -> Post {
    use schema::posts::dsl::*;

    posts
        .find(post_id)
        .first(conn)
        .expect("Error select post")
}

pub fn create_post<S: Into<String>>(conn: &SqliteConnection, title: S, user_id: i32) {
    use schema::posts;

    let new_post = NewPost {
        title: title.into(),
        user_id: user_id,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error create post");
}

pub fn delete_post(conn: &SqliteConnection, post_id: i32) {
    use schema::posts::dsl::*;

    diesel::delete(
        posts.find(post_id) 
    )
        .execute(conn)
        .expect("Error delete post");
}

pub fn select_tag(conn: &SqliteConnection, tag_id: i32) -> Tag {
    use schema::tags::dsl::*;

    tags
        .find(tag_id)
        .first(conn)
        .expect("Error select tag")
}

pub fn create_tag<S: Into<String>>(conn: &SqliteConnection, label: S) {
    use schema::tags;

    let new_tag = NewTag {
        label: label.into(),
    };

    diesel::insert_into(tags::table)
        .values(&new_tag)
        .execute(conn)
        .expect("Error create tag");
}

pub fn delete_tag(conn: &SqliteConnection, tag_id: i32) {
    use schema::tags::dsl::*;

    diesel::delete(
        tags.find(tag_id) 
    )
        .execute(conn)
        .expect("Error delete tag");
}

pub fn select_post_tag(conn: &SqliteConnection, post_tag_id: i32) -> PostTag {
    use schema::posts_tags::dsl::*;

    posts_tags
        .find(post_tag_id)
        .first(conn)
        .expect("Error select post_tag")
}

pub fn create_post_tag(conn: &SqliteConnection, post_id: i32, tag_id: i32) {
    use schema::posts_tags;

    let new_post_tag = NewPostTag {
        post_id: post_id,
        tag_id: tag_id,
    };

    diesel::insert_into(posts_tags::table)
        .values(&new_post_tag)
        .execute(conn)
        .expect("Error create post_tag");
}

pub fn delete_post_tag(conn: &SqliteConnection, post_tag_id: i32) {
    use schema::posts_tags::dsl::*;

    diesel::delete(
        posts_tags.find(post_tag_id) 
    )
        .execute(conn)
        .expect("Error delete post_tag");
}

pub fn select_posts_by_user(conn: &SqliteConnection, user_id: i32) -> Vec<Post> {
    let user = select_user(conn, user_id);

    Post::belonging_to(&user)
        .load::<Post>(conn)
        .expect("Error loading posts")
}

pub fn select_posts_by_tag(conn: &SqliteConnection, tag_id: i32) -> Vec<Post> {
    use schema::posts;
    use schema::posts_tags;

    let ids = posts_tags::table
        .filter(posts_tags::tag_id.eq(tag_id))
        .select(posts_tags::post_id)
        .load::<i32>(conn)
        .expect("Error loading ids");

    posts::table
        .filter(posts::id.eq_any(ids))
        .load::<Post>(conn)
        .expect("Error loading posts")
}

pub fn select_tags_by_post(conn: &SqliteConnection, post_id: i32) -> Vec<Tag> {
    use schema::tags;
    use schema::posts_tags;

    let ids = posts_tags::table
        .filter(posts_tags::post_id.eq(post_id))
        .select(posts_tags::tag_id)
        .load::<i32>(conn)
        .expect("Error loading ids");

    tags::table
        .filter(tags::id.eq_any(ids))
        .load::<Tag>(conn)
        .expect("Error loading tags")
}
