use crate::dap;
use crate::script::{Peers, ScriptInteraction, DAPScript};
use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

pub fn proxy_main(listen: &String, connect: &String, log_script: bool) -> io::Result<()> {
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
            run_proxy(ide_stream, da_stream, log_script)?;
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    drop(server);
    Ok(())
}

fn run_proxy(ide: TcpStream, da: TcpStream, log_script: bool) -> io::Result<()> {
    let (log_da, log_rx) = mpsc::channel::<ScriptInteraction>();
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
                match pipe_dap(&mut da, &mut ide, &log_da, Peers::Da) {
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
            match pipe_dap(&mut ide, &mut da, &log_ide, Peers::Ide) {
                Err(_) => break,
                _ => (),
            }
        }
    });

    let mut script = DAPScript {
        interactions: Vec::new(),
    };
    for received in log_rx {
        let prefix = match received.source {
            Peers::Ide => "ide -> ",
            Peers::Da => "ide <- ",
        };
        if log_script {
            script.interactions.push(received.clone());
        }
        println!("{}{}", prefix, received.content);
    }
    if log_script {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open("dap.log")?;
        file.write_all(json::stringify_pretty(&script, 4).as_bytes())?;
    }
    Ok(())
}

fn pipe_dap(
    from: &mut dyn Read,
    to: &mut dyn Write,
    log_tx: &mpsc::Sender<ScriptInteraction>,
    sender: Peers,
) -> std::io::Result<()> {
    let message = dap::read_message(from)?;
    let interaction = ScriptInteraction {
        source: sender,
        content: message.content.clone(),
    };
    log_tx.send(interaction).unwrap();
    dap::send_message(to, &message.content)?;
    Ok(())
}
