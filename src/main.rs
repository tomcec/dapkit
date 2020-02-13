mod dap;
mod script;
use json::object;
use std::io::prelude::*;
use std::net::TcpListener;
use script::*;

fn main() -> std::io::Result<()> {
    let script = load_script("session_01.dap")?;
    println!("Script: {:?}", script);

    // It's possible that we will initialize connection here.
    let listener = TcpListener::bind("127.0.0.1:3333")?;
    for stream in listener.incoming() {
        let mut io = stream?;
        io.set_read_timeout(Some(std::time::Duration::new(10, 0)))?;

        // 1. if script says send something - do it now.
        // 2. Wait for message
        // 3. Match message to expected in script
        // 3.1 If no response found - stop
        // 4. goto 1

        let msg: dap::DapMessage = dap::read_message(&mut io)?;
        io.write_all(
            json::stringify(object! {
                "header" => msg.header,
                "content" => msg.content
            })
            .as_bytes(),
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1 + 1, 2);
    }
}
