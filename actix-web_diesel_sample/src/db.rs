use actix::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;
use r2d2::{Pool};
use r2d2_diesel::{ConnectionManager};

use models;
use schema;

pub struct DbExecutor(pub Pool<ConnectionManager<SqliteConnection>>);

pub struct CreateUser {
    pub name: String,
}

impl Message for CreateUser {
    type Result = Result<models::User, Error>;
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<CreateUser> for DbExecutor {
    type Result = Result<models::User, Error>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        use self::schema::users::dsl::*;

        let new_user = models::NewUser {
            name: &msg.name,
        };

        let conn: &SqliteConnection = &self.0.get().unwrap();

        diesel::insert_into(users)
            .values(&new_user)
            .execute(conn)
            .map_err(|_| error::ErrorInternalServerError("Error inserting person"))?;

        let mut items = users
            .filter(name.eq(&msg.name))
            .load::<models::User>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading person"))?;

        Ok(items.pop().unwrap())
    }
}
