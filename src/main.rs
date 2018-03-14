extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate erased_serde;

#[macro_use]
extern crate serde_derive;

mod app;

fn main() {
    app::start("127.0.0.1:3012".to_string()).unwrap();
}