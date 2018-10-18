use schema::{users, posts, tags, posts_tags};

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub user_id: i32,
}

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "tags"]
pub struct Tag {
    pub id: i32,
    pub label: String,
}

#[derive(Insertable)]
#[table_name="tags"]
pub struct NewTag {
    pub label: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Post)]
#[belongs_to(Tag)]
#[table_name = "posts_tags"]
pub struct PostTag {
    pub id: i32,
    pub post_id: i32,
    pub tag_id: i32,
}

#[derive(Insertable)]
#[table_name="posts_tags"]
pub struct NewPostTag {
    pub post_id: i32,
    pub tag_id: i32,
}

