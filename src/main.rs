mod dap;
mod script;
use clap::Clap;
use script::*;

use std::net::{SocketAddr, TcpListener};

#[derive(Clap)]
#[clap(version = "1.0")]
struct Opts {
    /// Sets a custom script file.
    #[clap(short = "s", long = "script", default_value = "default.dap")]
    script: String,
    /// TSP server mode
    #[clap(short = "l")]
    server: bool,
    /// Port to listen.
    #[clap(short = "p", long = "port", default_value = "3333")]
    port: u16,
}

fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();
    let script = load_script(&opts.script)?;
    println!("Script: {:?}", script);

    if opts.server {
        let addr = SocketAddr::from(([127, 0, 0, 1], opts.port));
        let listener = TcpListener::bind(addr)?;
        for stream in listener.incoming() {
            let mut io = stream?;
            let mut input = io.try_clone()?;
            io.set_read_timeout(Some(std::time::Duration::new(10, 0)))?;
            script.run_script(&mut input, &mut io, script::Peers::Da);
        }
    } else {
        script.run_script(&mut std::io::stdin(), &mut std::io::stdout(), script::Peers::Da);
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
