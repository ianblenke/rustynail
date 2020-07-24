extern crate rustynail;
extern crate diesel;

use self::diesel::prelude::*;
use self::rustynail::*;
use std::env::args;

fn main() {
    use rustynail::schema::nails::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(nails.filter(sha512.like(pattern)))
        .execute(&connection)
        .expect("Error deleting nails");

    println!("Deleted {} nails", num_deleted);
}
