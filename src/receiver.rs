use std::{
    fs::File,
    io::Write,
    net::{TcpListener, TcpStream, UdpSocket},
    path::PathBuf,
};

use anyhow::Result;

use crate::{
    header::{Header, Operation},
    utils::{self, recv_content, recv_header},
};

pub fn receiver_main() -> Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", 0)).expect("Unable to open TcpListener");
    let listener_port = listener
        .local_addr()
        .expect("Unable to get local adress")
        .port();

    connect_to_sender(listener_port);

    for stream in listener.incoming() {
        let Ok(mut stream) = stream else {
            eprintln!("Connection failed. This is not fatal, but should reported");
            continue;
        };

        if let Err(e) = handle_client(&mut stream) {
            eprintln!("Failed to handle client: {e}")
        }
    }
    Ok(())
}

fn connect_to_sender(recv_port: u16) {
    let udp = UdpSocket::bind("0.0.0.0:0").expect("Unable to open UDP socket");
    udp.set_broadcast(true).expect("Set broadcast_call failed");

    let port = get_port();

    udp.send_to(&recv_port.to_be_bytes(), ("255.255.255.255", port))
        .expect("Couldn't send data");
}

fn get_port() -> u16 {
    loop {
        print!("Input connection code: ");
        match text_io::try_read!() {
            Ok(p) => return p,
            Err(e) => {
                println!("Couldn't parse input code ({e})")
            }
        }
    }
}

fn handle_client(stream: &mut TcpStream) -> Result<()> {
    let mut file_path: Option<PathBuf> = None;
    loop {
        let header = recv_header(stream)?;
        let content = recv_content(stream, header.length)?;
        let confirmation = Header {
            operation: Operation::RequestSucces,
            id: header.id,
            length: 0,
        };
        match header.operation {
            Operation::StartSendFile => {
                if file_path.is_some() {
                    anyhow::bail!("Received StartSendFile twice");
                }
                let mut f = PathBuf::new();
                f.push(if let Some(d) = dirs::document_dir() {
                    d
                } else {
                    anyhow::bail!("Couldn't find Document directory")
                });
                f.push(String::from_utf8(content)?);
                // println!("File will be saved at {f:?}");
                file_path = Some(f);
                utils::send_all(stream, confirmation, None)?;
            }
            Operation::SendFileContent => {
                if file_path.is_none() {
                    anyhow::bail!("No filename received before SendFileContent");
                }
                // println!("Receiving file content...");
                let mut file =
                    File::create(file_path.as_ref().expect("This should be unreachable"))?;
                file.write_all(&content)?;
                utils::send_all(stream, confirmation, None)?;
            }
            Operation::EndSendFile => {
                if header.length != 0 {
                    anyhow::bail!("Non zero content with EndSendFile")
                }
                // println!("End of file transmission");
                utils::send_all(stream, confirmation, None)?;

                break;
            }
            _ => anyhow::bail!("Invalid request from sender"),
        }
    }
    Ok(())
}
