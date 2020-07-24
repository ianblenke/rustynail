#![deny(warnings)]

use std::env;
use warp::Filter;

extern crate pretty_env_logger;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=debug` to see debug logs,
        // this only shows access logs.
       env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let hammer = warp::path("hammer")
        .and(warp::path::param())
        .map(|param: String| {
            let mut hasher = Sha256::new();
            hasher.input_str(&param);
            let hex = hasher.result_str();
            log::info!("{:?} <- {:?}", hex, param);
            Ok(format!("{}",hex))
        });

    let hammerhelp = warp::path("hammer")
        .and(warp::path::end())
        .map(|| "This is the rustynail hammer API. Try calling /hammer/{string}");

    let routes = warp::get().and(hammer.or(hammerhelp));

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
