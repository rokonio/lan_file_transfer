use std::{
    fs::File,
    io::{Read, Write},
    net::{SocketAddr, TcpStream, UdpSocket},
    path::Path,
};

pub fn sender_main() {
    let stream = connect_to_receiver();
    if let Err(e) = send_file(stream, "sender_data.txt") {
        panic!("The file was not able to be sent: {e:?}");
    }
}
fn connect_to_receiver() -> TcpStream {
    // Get a udp socket from OS (0.0.0.0:0 is a special ip that ask the OS for an unused ip and
    // port)
    let udp = UdpSocket::bind(("0.0.0.0", 0)).unwrap();
    let port = udp.local_addr().unwrap().port();
    println!("Connection code is {port}");
    let mut buf = [0; 2];
    // Loop until a valid response is received
    loop {
        let (_, addr) = udp.recv_from(&mut buf).unwrap();
        let port = u16::from_be_bytes(buf);
        let socker_addr = SocketAddr::new(addr.ip(), port);

        if let Ok(stream) = TcpStream::connect(socker_addr) {
            break stream;
        }
    }
}

fn send_file(mut stream: TcpStream, path: &str) -> std::io::Result<()> {
    let path = Path::new(path);
    let file_name = path.file_name().unwrap();
    println!("File name: {file_name:?}");

    let mut file = File::open(path)?;
    let file_size = file.metadata().unwrap().len();
    println!("File size: {file_size}");

    let mut content_buffer = vec![0; file_size as usize];
    let read_amt = file.read(&mut content_buffer)?;
    println!("Bytes read from file: {read_amt}");

    let written_amt = stream.write(&content_buffer)?;
    println!("Bytes written to stream: {written_amt}");
    Ok(())
}
