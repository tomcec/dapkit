use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

pub fn proxy_main(listen: &String, connect: &String) -> std::io::Result<()> {
    let server_adds: SocketAddr = listen
        .parse()
        .expect("Unable to parse server socket address");
    let connect_addr: SocketAddr = connect
        .parse()
        .expect("Unable to parse connect socket address");

    let server = TcpListener::bind(server_adds)?;
    println!("Listen on {}", server_adds);

    let stream = server.incoming().next().expect("No client");
    match stream {
        Ok(ide_stream) => {
            let da_stream = TcpStream::connect(connect_addr)?;
            println!("Connected to {}", connect_addr);
            run_proxy(ide_stream, da_stream)?;
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    drop(server);
    Ok(())
}

fn run_proxy(ide: TcpStream, da: TcpStream) -> std::io::Result<()> {
    let ide = Arc::new(ide);
    let da = Arc::new(da);
    {
        let ide = Arc::clone(&ide);
        let da = Arc::clone(&da);
        thread::spawn(move || {
            let mut da = da.try_clone().unwrap();
            let mut ide = ide.try_clone().unwrap();
            loop {
                // ida -> ide
                pipe_char(&mut da, &mut ide).expect("Error");
            }
        });
    }
    let mut ide = ide.try_clone().unwrap();
    let mut da = da.try_clone().unwrap();
    loop {
        // ide -> da
        pipe_char(&mut ide, &mut da)?;
    }
}

fn pipe_char(from: &mut dyn Read, to: &mut dyn Write) -> std::io::Result<()> {
    let mut ch: [u8; 1] = [0];
    from.read_exact(&mut ch[..])?;
    print!("{}", ch[0] as char);
    to.write(&ch)?;
    Ok(())
}
