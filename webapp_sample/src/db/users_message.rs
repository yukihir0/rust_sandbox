use bcrypt::{hash};
use chrono::*;
use uuid::Uuid;

use actix::prelude::*;
use actix_web::*;

use diesel;
use diesel::prelude::*;

use models;
use schema;
use db::{DbExecutor};

pub struct ReadUsers {}

impl Message for ReadUsers {
    type Result = Result<Vec<models::User>, Error>;
}

impl Handler<ReadUsers> for DbExecutor {
    type Result = Result<Vec<models::User>, Error>;

    fn handle(&mut self, _msg: ReadUsers, _: &mut Self::Context) -> Self::Result {
        use self::schema::users::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        let select_users = users
            .load::<models::User>(conn)
            .map_err(|e| {
                match e {
                    diesel::result::Error::NotFound => error::ErrorNotFound("NotFound"),
                    _ => error::ErrorInternalServerError("InternalServerError"),
                }
            })?;

        Ok(select_users)
    }
}

pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl Message for CreateUser {
    type Result = Result<models::User, Error>;
}

impl Handler<CreateUser> for DbExecutor {
    type Result = Result<models::User, Error>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        use self::schema::users::dsl::*;

        let digest = hash(&msg.password, 5).unwrap();
       
        let now = Local::now().naive_local();

        let new_user = models::NewUser {
            uuid: &Uuid::new_v4().to_string(),
            name: &msg.name,
            email: &msg.email,
            password_digest: &digest,
            created_at: now,
            updated_at: now,
        };

        let conn: &SqliteConnection = &self.0.get().unwrap();

        diesel::insert_into(users)
            .values(new_user)
            .execute(conn)
            .map_err(|e| {
                match e {
                    diesel::result::Error::NotFound => error::ErrorNotFound("NotFound"),
                    _ => error::ErrorInternalServerError("InternalServerError"),
                }
            })?;

        let insert_user = users
            .filter(email.eq(&msg.email))
            .first(conn)
            .map_err(|e| {
                match e {
                    diesel::result::Error::NotFound => error::ErrorNotFound("NotFound"),
                    _ => error::ErrorInternalServerError("InternalServerError"),
                }
            })?;

        Ok(insert_user)
    }
}

pub struct ReadUser {
    pub id: i32,
}

impl Message for ReadUser {
    type Result = Result<models::User, Error>;
}

impl Handler<ReadUser> for DbExecutor {
    type Result = Result<models::User, Error>;

    fn handle(&mut self, msg: ReadUser, _: &mut Self::Context) -> Self::Result {
        use self::schema::users::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        let select_user = users
            .find(msg.id)
            .first(conn)
            .map_err(|e| {
                match e {
                    diesel::result::Error::NotFound => error::ErrorNotFound("NotFound"),
                    _ => error::ErrorInternalServerError("InternalServerError"),
                }
            })?;

        Ok(select_user)
    }
}

pub struct UpdateUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl Message for UpdateUser {
    type Result = Result<models::User, Error>;
}

impl Handler<UpdateUser> for DbExecutor {
    type Result = Result<models::User, Error>;

    fn handle(&mut self, msg: UpdateUser, _: &mut Self::Context) -> Self::Result {
        use self::schema::users::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        let digest = hash(&msg.password, 5).unwrap();
        
        diesel::update(users
            .find(msg.id))
            .set((
                name.eq(&msg.name),
                email.eq(&msg.email),
                password_digest.eq(&digest),
                updated_at.eq(Local::now().naive_local()),
            ))
            .execute(conn)
            .map_err(|e| {
                match e {
                    diesel::result::Error::NotFound => error::ErrorNotFound("NotFound"),
                    _ => error::ErrorInternalServerError("InternalServerError"),
                }
            })?;

        let update_user = users
            .find(msg.id)
            .first(conn)
            .map_err(|e| {
                match e {
                    diesel::result::Error::NotFound => error::ErrorNotFound("NotFound"),
                    _ => error::ErrorInternalServerError("InternalServerError"),
                }
            })?;

        Ok(update_user)
    }
}

pub struct DeleteUser {
    pub id: i32,
}

impl Message for DeleteUser {
    type Result = Result<models::User, Error>;
}

impl Handler<DeleteUser> for DbExecutor {
    type Result = Result<models::User, Error>;

    fn handle(&mut self, msg: DeleteUser, _: &mut Self::Context) -> Self::Result {
        use self::schema::users::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        let delete_user = users
            .find(msg.id)
            .first(conn)
            .map_err(|e| {
                match e {
                    diesel::result::Error::NotFound => error::ErrorNotFound("NotFound"),
                    _ => error::ErrorInternalServerError("InternalServerError"),
                }
            })?;

        diesel::delete(users
            .find(msg.id))
            .execute(conn)
            .map_err(|e| {
                match e {
                    diesel::result::Error::NotFound => error::ErrorNotFound("NotFound"),
                    _ => error::ErrorInternalServerError("InternalServerError"),
                }
            })?;

        Ok(delete_user)
    }
}

pub struct ReadUserByEmail {
    pub email: String,
}

impl Message for ReadUserByEmail {
    type Result = Result<models::User, Error>;
}

impl Handler<ReadUserByEmail> for DbExecutor {
    type Result = Result<models::User, Error>;

    fn handle(&mut self, msg: ReadUserByEmail, _: &mut Self::Context) -> Self::Result {
        use self::schema::users::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        let select_user = users
            .filter(email.eq(&msg.email))
            .first(conn)
            .map_err(|e| {
                match e {
                    diesel::result::Error::NotFound => error::ErrorNotFound("NotFound"),
                    _ => error::ErrorInternalServerError("InternalServerError"),
                }
            })?;

        Ok(select_user)
    }
}

pub struct UpdateUserSession {
    pub id: i32,
    pub session_id: String,
}

impl Message for UpdateUserSession {
    type Result = Result<models::User, Error>;
}

impl Handler<UpdateUserSession> for DbExecutor {
    type Result = Result<models::User, Error>;

    fn handle(&mut self, msg: UpdateUserSession, _: &mut Self::Context) -> Self::Result {
        use self::schema::users::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        let digest = hash(&msg.session_id, 5).unwrap();
        
        diesel::update(users
            .find(msg.id))
            .set((
                session_digest.eq(&digest),
                updated_at.eq(Local::now().naive_local()),
            ))
            .execute(conn)
            .map_err(|e| {
                match e {
                    diesel::result::Error::NotFound => error::ErrorNotFound("NotFound"),
                    _ => error::ErrorInternalServerError("InternalServerError"),
                }
            })?;

        let update_user = users
            .find(msg.id)
            .first(conn)
            .map_err(|e| {
                match e {
                    diesel::result::Error::NotFound => error::ErrorNotFound("NotFound"),
                    _ => error::ErrorInternalServerError("InternalServerError"),
                }
            })?;

        Ok(update_user)
    }
}
