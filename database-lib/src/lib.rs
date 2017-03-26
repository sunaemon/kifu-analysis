// diesel crates
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn establish_connection() -> PgConnection {
    use std::env;
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
