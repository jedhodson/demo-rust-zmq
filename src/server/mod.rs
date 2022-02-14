extern crate serde_derive;
extern crate rmp_serde as rmps;
use serde::{ Serialize, Deserialize};
use rmps::{Serializer, Deserializer};
use std::thread;
use std::time::Duration;

fn init_pipe() -> zmq::Socket {
    let context = zmq::Context::new();
    let requester = context.socket(zmq::REP).unwrap();
    assert!(requester.bind("ipc:///home/jedhodson/tmp/pipe").is_ok());

    requester
}

use crate::types::TemperatureHumiditySensor;
pub fn entry_point() {
    println!("Starting server...");
    let responder = init_pipe();
    let mut msg = zmq::Message::new();

    loop {
        responder.recv(&mut msg, 0).unwrap();
        println!("Received {}", msg.as_str().unwrap());
        thread::sleep(Duration::from_millis(1000));

        let data_out = TemperatureHumiditySensor {
            address: 0x18,
            temperature: 25,
            humidity: 64.5
        };

        let mut buf = Vec::new();
        data_out.serialize(&mut Serializer::new(&mut buf)).unwrap();
        println!("Sending: {:?}\nEncoded: {:x?}", data_out, buf);
        responder.send(buf, 0).unwrap();
    }
}