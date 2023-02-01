use std::{
    fs::File,
    io::{Read, Write},
    net::TcpStream,
    path::Path,
};

use crate::consts::*;

pub fn sender_main() {
    let stream = TcpStream::connect(("127.0.0.1", PORT)).unwrap();

    if let Err(e) = send_file(stream, "sender_data.txt") {
        panic!("The file was not able to be sent: {e:?}");
    }
}

fn send_file(mut stream: TcpStream, path: &str) -> std::io::Result<()> {
    let path = Path::new(path);
    let file_name = path.file_name().unwrap();
    println!("File name: {file_name:?}");

    let mut file = File::open(path)?;
    let file_size = file.metadata().unwrap().len();
    println!("File size: {file_size}");

    let mut buffer = vec![0; file_size as usize];
    let read_amt = file.read(&mut buffer)?;
    println!("Bytes read from file: {read_amt}");

    let written_amt = stream.write(&buffer)?;
    println!("Bytes written to stream: {written_amt}");
    Ok(())
}
