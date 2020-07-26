#![deny(warnings)]

pub mod schema;
pub mod db_connection;
pub mod models;
pub mod handlers;

//#[macro_use]
//extern crate diesel_migrations;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use] 
extern crate serde_derive;
extern crate pretty_env_logger;

extern crate actix;
extern crate actix_web;
extern crate futures;
use std::env;
use actix_web::{App, HttpServer, web};
use db_connection::establish_connection;

//embed_migrations!("migrations");

fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=debug` to see debug logs,
        // this only shows access logs.
       env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    //let pool = establish_connection();
    //let connection = pool.get().unwrap();
    //let _result = embedded_migrations::run_with_output(&connection, &mut std::io::stdout());

    let sys = actix::System::new("rustynail");

    HttpServer::new(
    || App::new()
        .data(establish_connection())
        .service(
            web::resource("/nail")
                .route(web::get().to_async(handlers::nails::index))
                .route(web::post().to_async(handlers::nails::create))
        )
        .service(
            web::resource("/nail/{id}")
                .route(web::get().to_async(handlers::nails::show))
                .route(web::delete().to_async(handlers::nails::destroy))
                .route(web::patch().to_async(handlers::nails::update))
        )
    )
    .bind("0.0.0.0:3030").unwrap()
    .start();

    println!("Started http server: 0.0.0.0:3030");
    let _ = sys.run();
}
