mod script;
mod dap;
use clap::Clap;
use script::*;

use std::net::TcpListener;

#[derive(Clap)]
#[clap(version = "1.0")]
struct Opts {
    /// Sets a custom script file.
    #[clap(short = "s", long = "script", default_value = "default.dap")]
    script: String,
}

fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();
    let script = load_script(&opts.script)?;
    println!("Script: {:?}", script);

    // It's possible that we will initialize connection here.
    let listener = TcpListener::bind("127.0.0.1:3333")?;
    for stream in listener.incoming() {
        let mut io = stream?;
        io.set_read_timeout(Some(std::time::Duration::new(10, 0)))?;
        script.run_script(&mut io, script::Peers::Da);
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
