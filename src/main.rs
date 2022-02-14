use std::env;

// Serialization Stuff
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;
use serde::{ Serialize, Deserialize};
use rmps::{Serializer, Deserializer};

// Application code
mod types;
mod server;
mod client;


fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len() > 1, true);

    let cb = match args[1].as_str() {
        "client" => client::entry_point,
        "server" => server::entry_point,
        _ => || {
            println!("Option Not Supported!")
        }
    };

    cb();
}
