use std::{
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream, UdpSocket},
    path::PathBuf,
};

use anyhow::Result;

use crate::header::Header;

pub fn receiver_main() -> std::io::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", 0))?;
    let listener_port = listener.local_addr().unwrap().port();

    connect_to_sender(listener_port);

    for stream in listener.incoming() {
        handle_client(&mut stream.unwrap()).unwrap();
    }
    Ok(())
}

fn connect_to_sender(recv_port: u16) {
    let udp = UdpSocket::bind("0.0.0.0:0").unwrap();
    udp.set_broadcast(true).unwrap();

    println!("Input connection code: ");
    let port: u16 = text_io::read!();

    for _ in 0..10 {
        udp.send_to(&recv_port.to_be_bytes(), ("255.255.255.255", port))
            .unwrap();
    }
}

fn handle_client(stream: &mut TcpStream) -> Result<()> {
    let mut file_path: Option<PathBuf> = None;
    loop {
        let Ok(header) = read_header(stream) else {
            anyhow::bail!("Error decoding header")
        };
        let mut content: Vec<u8> = Vec::with_capacity(header.length as usize);
        if header.length > 0 {
            stream
                .take(header.length as u64)
                .read_to_end(&mut content)?;
        }

        match header.operation {
            crate::header::Operation::StartSendFile => {
                // let name = String::from_utf8(content)?;
                // println!("Receiving file named: {name}")
                if file_path.is_some() {
                    anyhow::bail!("Received StartSendFile twice");
                }
                let mut f = PathBuf::new();
                f.push(dirs::document_dir().expect("Couldn't find Document directory"));
                f.push(String::from_utf8(content)?);
                println!("File will be saved at {f:?}");
                file_path = Some(f);
            }
            crate::header::Operation::SendFileContent => {
                if file_path.is_none() {
                    anyhow::bail!("No filename received before SendFileContent");
                }
                println!("Receiving file content...");
                let mut file = File::create(file_path.as_ref().unwrap())?;
                file.write_all(&content)?;
            }
            crate::header::Operation::EndSendFile => {
                if header.length != 0 {
                    anyhow::bail!("Non zero content with EndSendFile")
                }
                println!("End of file transmission");
                break;
            }
            crate::header::Operation::RequestSucces => todo!(),
            crate::header::Operation::RequestRefuse => todo!(),
        }
    }
    Ok(())

    // let mut buffer = String::new();
    //
    // println!("bytes read: {bytes_read}");
    // println!("Buffer currently holds: {buffer}");
}

fn read_header(stream: &mut TcpStream) -> Result<Header> {
    let mut buf = Vec::with_capacity(Header::SIZE);
    stream.take(Header::SIZE as u64).read_to_end(&mut buf)?;
    let [l1, l2, l3, l4, l5] = buf.get(0..5).unwrap() else {unreachable!()};

    Header::decode([*l1, *l2, *l3, *l4, *l5])
}
