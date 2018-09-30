pub mod users_message;

use actix::prelude::*;

use diesel::prelude::*;
use r2d2::{Pool};
use r2d2_diesel::{ConnectionManager};

pub struct DbExecutor(
    pub Pool<ConnectionManager<SqliteConnection>>
);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}
