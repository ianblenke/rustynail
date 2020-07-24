#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
//use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use self::models::NewNail;

//pub fn establish_connection() -> PgConnection {
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
//    PgConnection::establish(&database_url)
//        .expect(&format!("Error connecting to {}", database_url))
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_nail(conn: &SqliteConnection, sha512: &str, param: &str) -> usize {
    use crate::schema::nails;

    let new_nail = NewNail { sha512, param };

    diesel::insert_into(nails::table)
        .values(&new_nail)
        .execute(conn)
        .expect("Error saving new nail")
}
