use crate::script::Peers;
use crate::script::*;
use std::net::{SocketAddr, TcpListener};

pub fn mock_main(script_name: &String, pipes: bool, port: u16) -> std::io::Result<()> {
    let script = load_script(script_name)?;
    println!("{} loaded with {} steps", script_name, script.interactions.len());
    if pipes {
        script.run_script(&mut std::io::stdin(), &mut std::io::stdout(), Peers::Da);
    } else {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        let listener = TcpListener::bind(addr)?;
        println!("Listen on {}", addr);
        let stream = listener.incoming().next().unwrap();
        let mut io = stream?;
        let mut input = io.try_clone()?;
        script.run_script(&mut input, &mut io, Peers::Da);
    }
    Ok(())
}
