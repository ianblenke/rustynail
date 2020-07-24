use rustynail::*;
use std::io::{stdin};
use crypto::digest::Digest;
use crypto::sha2::Sha512;

fn main() {
    let connection = establish_connection();

    println!("What would you like your param to be?");
    let mut param = String::new();
    stdin().read_line(&mut param).unwrap();
    let param = &param[..(param.len() - 1)]; // Drop the newline character

    let mut hasher = Sha512::new();
    hasher.input_str(&param);
    let sha512 = hasher.result_str();

    let _ = create_nail(&connection, &sha512, &param);
    println!("\nSaved nail {}", sha512);
}
