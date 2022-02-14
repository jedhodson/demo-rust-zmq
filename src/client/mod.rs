extern crate serde_derive;
extern crate rmp_serde as rmps;
use serde::{ Serialize, Deserialize};
use rmps::{Serializer, Deserializer};

fn init_pipe() -> zmq::Socket {
    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();
    assert!(requester.connect("ipc:///home/jedhodson/tmp/pipe").is_ok());

    requester
}

use crate::types::TemperatureHumiditySensor;
pub fn entry_point() {
    println!("Starting client...");
    let requester = init_pipe();
    let mut msg = zmq::Message::new();

    for _ in 0..10 {
        println!("Sending...");
        requester.send("Hello", 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        let mut de = Deserializer::new(&msg[..]);
        let result: TemperatureHumiditySensor = Deserialize::deserialize(&mut de).unwrap();
        println!("Received {:x?}\nDecoded: {:?}", msg, result);
    }
}