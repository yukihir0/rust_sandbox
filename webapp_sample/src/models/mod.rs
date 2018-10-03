use super::schema::users;
use chrono::{DateTime, NaiveDateTime, Local};

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub email: String,
    pub password_digest: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub uuid: &'a str,
    pub name: &'a str,
    pub email: &'a str,
    pub password_digest: &'a str,
    pub created_at: NaiveDateTime,
}
