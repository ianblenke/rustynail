extern crate rustynail;
extern crate diesel;

use self::rustynail::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use rustynail::schema::nails::dsl::*;

    let connection = establish_connection();
    let results = nails.limit(5)
        .load::<Nail>(&connection)
        .expect("Error loading nails");

    println!("Displaying {} nails", results.len());
    for nail in results {
        println!("{}", nail.sha512);
        println!("----------\n");
        println!("{}", nail.param);
    }
}
