#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
//use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError };

//use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use self::models::NewNail;

/*
pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> PgPool {
*/
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

//    init_pool(&database_url).expect("Failed to create pool")
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))

}

//pub fn create_nail(conn: &SqliteConnection, sha512: &str, param: &str) -> usize {
pub fn create_nail(conn: &PgConnection, sha512: &str, param: &str) -> usize {
    use crate::schema::nails;

    let new_nail = NewNail { sha512, param };

    diesel::insert_into(nails::table)
        .values(&new_nail)
        .execute(conn)
        .expect("Error saving new nail")
}
