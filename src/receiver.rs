use std::{
    io::Read,
    net::{TcpListener, TcpStream, UdpSocket},
};

use crate::consts::*;

pub fn receiver_main() -> std::io::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", 0))?;
    let l_port = listener.local_addr().unwrap().port();

    let udp = UdpSocket::bind("0.0.0.0:0").unwrap();
    udp.set_broadcast(true).unwrap();

    println!("Input sender port: ");
    let port: u16 = text_io::read!();

    for _ in 0..10 {
        udp.send_to(&l_port.to_be_bytes(), ("255.255.255.255", port))
            .unwrap();
    }

    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = vec![0; BUFFER_SIZE];
    let bytes_read = stream.read(&mut buffer).unwrap();
    println!("bytes read: {bytes_read}");
    println!(
        "Buffer currently holds: {:?}",
        std::str::from_utf8(&buffer[0..bytes_read]).unwrap()
    );
}
