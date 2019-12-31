use json::{array, object};
use std::net::TcpListener;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let json = object! {
    "code" => 200,
    "success" => true,
    "payload" => object!{
        "features" => array![
            "awesome",
            "easyAPI",
            "lowLearningCurve"
            ]
        }
    };

    let listener = TcpListener::bind("127.0.0.1:3333")?;
    let json_str = json::stringify(json);
    for stream in listener.incoming() {
        stream?.write_all(json_str.as_bytes());
    }
    Ok(())
}
