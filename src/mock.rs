use crate::script::Peers;
use crate::script::*;
use std::net::{SocketAddr, TcpListener, TcpStream};

pub fn mock_main(
    script_name: &String,
    pipes: bool,
    address: &String,
    role: Peers,
) -> std::io::Result<()> {
    let script = load_script(script_name)?;
    println!(
        "{} loaded with {} steps",
        script_name,
        script.interactions.len()
    );
    if pipes {
        script.run_script(&mut std::io::stdin(), &mut std::io::stdout(), role);
    } else {
        let addr: SocketAddr = address.parse().expect("Unable to parse socket address");
        let mut io = open_stream(&addr, role);
        let mut input = io.try_clone()?;
        script.run_script(&mut input, &mut io, role);
    }
    Ok(())
}

fn open_stream(address: &SocketAddr, role: Peers) -> TcpStream {
    if role == Peers::Da {
        let listener = TcpListener::bind(address).unwrap();
        println!("Listen on {}", address);
        return listener.incoming().next().unwrap().unwrap();
    } else {
        let da_stream = TcpStream::connect(address).unwrap();
        println!("Connected to {}", address);
        return da_stream;
    }
}
