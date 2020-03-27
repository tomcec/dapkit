use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc;
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
    let (log_da, log_rx) = mpsc::channel::<String>();
    let log_ide = mpsc::Sender::clone(&log_da);
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
                match pipe_dap(&mut da, &mut ide, &log_da, "ide <- ") {
                    Err(_) => break,
                    _ => (),
                }
            }
        });
    }
    let mut ide = ide.try_clone().unwrap();
    let mut da = da.try_clone().unwrap();
    thread::spawn(move || {
        loop {
            // ide -> da
            match pipe_dap(&mut ide, &mut da, &log_ide, "ide -> ") {
                Err(_) => break,
                _ => (),
            }
        }
    });
    for received in log_rx {
        println!("{}", received);
    }
    Ok(())
}

fn pipe_dap(from: &mut dyn Read, to: &mut dyn Write, log_tx: &mpsc::Sender<String>, prefix: &str) -> std::io::Result<()> {
    let message = crate::dap::read_message(from)?;
    log_tx.send(format!("{}{}", prefix, message)).unwrap();
    crate::dap::send_message(to, &message.content)?;
    Ok(())
}
