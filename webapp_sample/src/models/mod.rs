use super::schema::users;
use chrono::{NaiveDateTime};

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub email: String,
    pub password_digest: String,
    pub session_digest: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub uuid: &'a str,
    pub name: &'a str,
    pub email: &'a str,
    pub password_digest: &'a str,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserSession {
    pub user_id: i32,
    pub session_id: String,
}
