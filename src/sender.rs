use std::{
    fs::File,
    io::Read,
    net::{SocketAddr, TcpStream, UdpSocket},
    path::Path,
    sync::Mutex,
};

use anyhow::Result;

use crate::header::{Header, Operation};
use crate::utils;

static ID: Mutex<u16> = Mutex::new(1);

fn increment_id() {
    let mut id = ID.lock().expect("Unable to aquire id counter");
    *id += 1;
}

pub fn new_id() -> u16 {
    let id = *ID.lock().expect("Unable to aquire id counter");
    increment_id();
    id
}

pub fn sender_main() {
    let stream = connect_to_receiver();
    if let Err(e) = send_file(stream, "sender_data.txt") {
        panic!("The file was not able to be sent: {e:?}");
    }
}
fn connect_to_receiver() -> TcpStream {
    // Get a udp socket from OS (0.0.0.0:0 is a special ip that ask the OS for an unused ip and
    // port)
    let udp = UdpSocket::bind(("0.0.0.0", 0)).expect("Unable to open UDP Socket");
    let port = udp.local_addr().expect("Couldn't get local adress").port();
    println!("Connection code is {port}");
    let mut buf = [0; 2];
    // Loop until a valid response is received
    loop {
        let Ok((_, addr)) = udp.recv_from(&mut buf) else {
            continue;
        };
        let port = u16::from_be_bytes(buf);
        let socker_addr = SocketAddr::new(addr.ip(), port);

        if let Ok(stream) = TcpStream::connect(socker_addr) {
            break stream;
        }
    }
}

fn send_file(mut stream: TcpStream, path: &str) -> Result<()> {
    let path = Path::new(path);
    let mut file = File::open(path)?;

    let file_name = path.file_name().expect("Input is not a file");
    let file_size = file.metadata().expect("Couldn't get file metadata").len();

    let mut content_buffer = vec![0; file_size as usize];
    let read_amt = file.read(&mut content_buffer)?;
    if read_amt != file_size as usize {
        eprintln!("Read amount differs from file size. This is not fatal, but should be reported");
    }

    send_name(
        &mut stream,
        new_id(),
        file_name
            .to_str()
            .expect("Couldn't convert file name to UTF-8"),
    )?;
    send_content(&mut stream, new_id(), content_buffer)?;
    end_connection(&mut stream, new_id())?;
    Ok(())
}

fn send_name(stream: &mut TcpStream, id: u16, name: &str) -> Result<()> {
    let name_bytes = name.as_bytes();
    let header = Header {
        operation: Operation::StartSendFile,
        length: name_bytes.len() as u32,
        id,
    };
    utils::send_all(stream, header, Some(name_bytes))?;
    if !recv_confirmation(stream, id)? {
        anyhow::bail!("Receiver refused to take file name");
    }
    Ok(())
}
fn send_content(stream: &mut TcpStream, id: u16, content: Vec<u8>) -> Result<()> {
    let header = Header {
        operation: Operation::SendFileContent,
        length: content.len() as u32,
        id,
    };
    utils::send_all(stream, header, Some(&content))?;
    if !recv_confirmation(stream, id)? {
        anyhow::bail!("Receiver refused to take file content");
    }
    Ok(())
}
fn end_connection(stream: &mut TcpStream, id: u16) -> Result<()> {
    let header = Header {
        operation: Operation::EndSendFile,
        length: 0,
        id,
    };
    utils::send_all(stream, header, None)?;
    if !recv_confirmation(stream, id)? {
        anyhow::bail!("Receiver refused to take file closing instruction");
    }
    Ok(())
}

fn recv_confirmation(stream: &mut TcpStream, id: u16) -> Result<bool> {
    let header = utils::recv_header(stream)?;
    let success = match header.operation {
        Operation::RequestSucces => true,
        Operation::RequestRefuse => false,
        other => anyhow::bail!("Invalid reponse from receiver: {other:?}"),
    };
    if header.length > 0 {
        let content = utils::recv_content(stream, header.length)?;
        eprintln!(
            "Content received with confirmation: {}",
            String::from_utf8_lossy(&content)
        );
    }
    if id != header.id {
        anyhow::bail!("Wrong id");
    }
    Ok(success)
}
