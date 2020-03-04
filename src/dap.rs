use std::io::prelude::*;

#[derive(Debug)]
pub struct DapMessage {
    pub header: i64,
    pub content: String,
}

fn read_header(stream: &mut dyn Read) -> Result<i64, std::io::Error> {
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
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Wrong header",
    ))
}

pub fn read_message(stream: &mut dyn Read) -> Result<DapMessage, std::io::Error> {
    let header: i64 = read_header(stream)?;
    let mut buf = vec![0u8; header as usize];
    stream.read_exact(&mut buf)?;
    let msg: String = String::from_utf8(buf).unwrap();
    Ok(DapMessage {
        header: header,
        content: msg,
    })
}

pub fn send_message(stream: &mut dyn Write, content: &String) {
    stream.write_all(format!("Content-Length:{}\n\r", content.len()).as_bytes()).unwrap();
    stream.write_all(content.as_bytes()).unwrap();
}
