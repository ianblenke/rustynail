# rustynail

A quick and dirty little web service that takes a URL string parameter and returns the sha512 hash.

Built on rust [warp](https://github.com/seanmonstar/warp)

## usage:

    cargo build
    ./target/debug/rustynail

or just:

    cargo run --bin rustynail

The nails are currently serialized to a `database.sqlite3` file.

Changing this to PostgreSQL would merely require changing the `.env` file and uncommenting/commenting the lines in `src/lib.rs`

You can manually manipulate nails from the commandline using:

    cargo run --bin write_nail
    cargo run --bin delete_nail
    cargo run --bin show_nails

