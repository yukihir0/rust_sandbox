use actix::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;
use r2d2::{Pool};
use r2d2_diesel::{ConnectionManager};

use models;
use schema;

pub struct DbExecutor(pub Pool<ConnectionManager<SqliteConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub struct MessageReadUsers {}

impl Message for MessageReadUsers {
    type Result = Result<Vec<models::User>, Error>;
}

impl Handler<MessageReadUsers> for DbExecutor {
    type Result = Result<Vec<models::User>, Error>;

    fn handle(&mut self, _msg: MessageReadUsers, _: &mut Self::Context) -> Self::Result {
        use self::schema::users::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        let select_users = users
            .load::<models::User>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading users"))?;

        Ok(select_users)
    }
}

pub struct MessageCreateUser {
    pub name: String,
    pub email: String,
}

impl Message for MessageCreateUser {
    type Result = Result<models::User, Error>;
}

impl Handler<MessageCreateUser> for DbExecutor {
    type Result = Result<models::User, Error>;

    fn handle(&mut self, msg: MessageCreateUser, _: &mut Self::Context) -> Self::Result {
        use self::schema::users::dsl::*;

        let new_user = models::NewUser {
            name: &msg.name,
            email: &msg.email,
        };

        let conn: &SqliteConnection = &self.0.get().unwrap();

        diesel::insert_into(users)
            .values(&new_user)
            .execute(conn)
            .map_err(|_| error::ErrorInternalServerError("Error inserting user"))?;

        let mut items = users
            .filter(name.eq(&msg.name))
            .load::<models::User>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading user"))?;

        Ok(items.pop().unwrap())
    }
}

pub struct MessageReadUser {
    pub id: i32,
}

impl Message for MessageReadUser {
    type Result = Result<models::User, Error>;
}

impl Handler<MessageReadUser> for DbExecutor {
    type Result = Result<models::User, Error>;

    fn handle(&mut self, msg: MessageReadUser, _: &mut Self::Context) -> Self::Result {
        use self::schema::users::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        let select_user = users
            .find(msg.id)
            .first(conn)
            .map_err(|_| error::ErrorInternalServerError("Error select user"))?;

        Ok(select_user)
    }
}

pub struct MessageUpdateUser {
    pub id: i32,
    pub name: String,
    pub email: String
}

impl Message for MessageUpdateUser {
    type Result = Result<models::User, Error>;
}

impl Handler<MessageUpdateUser> for DbExecutor {
    type Result = Result<models::User, Error>;

    fn handle(&mut self, msg: MessageUpdateUser, _: &mut Self::Context) -> Self::Result {
        use self::schema::users::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        diesel::update(users.find(msg.id))
            .set((name.eq(msg.name), email.eq(msg.email)))
            .execute(conn)
            .map_err(|_| error::ErrorInternalServerError("Error update user"))?;

        let update_user = users
            .find(msg.id)
            .first(conn)
            .map_err(|_| error::ErrorInternalServerError("Error select user"))?;

        Ok(update_user)
    }
}

pub struct MessageDeleteUser {
    pub id: i32,
}

impl Message for MessageDeleteUser {
    type Result = Result<models::User, Error>;
}

impl Handler<MessageDeleteUser> for DbExecutor {
    type Result = Result<models::User, Error>;

    fn handle(&mut self, msg: MessageDeleteUser, _: &mut Self::Context) -> Self::Result {
        use self::schema::users::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        let delete_user = users
            .find(msg.id)
            .first(conn)
            .map_err(|_| error::ErrorInternalServerError("Error select user"))?;
        
        diesel::delete(users.filter(id.eq(msg.id)))
            .execute(conn)
            .map_err(|_| error::ErrorInternalServerError("Error delete user"))?;

        Ok(delete_user)
    }
}

