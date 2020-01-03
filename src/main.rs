mod dap;
use json::{array, object};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn read_header(stream: &mut TcpStream) -> Result<i64, std::io::Error> {
    let mut done = false;
    let mut header_bytes: Vec<u8> = Vec::new();
    while !done {
        let mut ch: [u8; 1] = [0];
        stream.read(&mut ch[..])?;
        if header_bytes.last() == Some(&b'\r') && ch[0] == b'\n' {
            done = true;
        }
        header_bytes.push(ch[0]);
    }
    let content_len = String::from_utf8(header_bytes).unwrap();
    if content_len.starts_with("Content-Length:") {
        let num = content_len["Content-Length:".len()..].trim();
        return Ok(num.parse::<i64>().unwrap());
    }
    Err(std::io::Error::new(std::io::ErrorKind::Other, "Wrong header"))
}

fn read_request(stream: &mut TcpStream) -> Result<dap::DapMessage, std::io::Error> {
    let mut msg: String = String::new();
    stream.set_read_timeout(Some(std::time::Duration::new(10, 0)))?;
    let header: i64 = read_header(stream)?;
    // stream.read_to_string(&mut msg)?;
    Ok(dap::DapMessage {
        header: header,
        content: msg,
    })
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
        let msg: dap::DapMessage = read_request(&mut io)?;
        io.write_all(
            json::stringify(object! {
                "header" => msg.header,
                "content" => msg.content
            })
            .as_bytes(),
        );
    }
    Ok(())
}
