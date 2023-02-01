use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use crate::consts::*;

pub fn receiver_main() -> std::io::Result<()> {
    let listener = TcpListener::bind(("127.0.0.1", PORT))?;

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
