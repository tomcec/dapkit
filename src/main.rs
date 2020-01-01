mod dap;
use json::{array, object};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn read_request(stream: &TcpStream) -> dap::DapMessage {
    dap::DapMessage {
        header: 3,
        content: "321".to_string()
    }
}

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
        let mut io = stream?;
        let msg: dap::DapMessage = read_request(&io);
        io.write_all(
            json::stringify(object! {
                "header" => msg.header,
                "content" => msg.content
            }).as_bytes());
    }
    Ok(())
}
