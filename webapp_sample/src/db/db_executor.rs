use diesel;
use diesel::prelude::*;
use r2d2::{Pool};
use r2d2_diesel::{ConnectionManager};

pub struct DbExecutor(
    pub Pool<ConnectionManager<SqliteConnection>>
);
