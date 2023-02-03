use std::{
    io::Read,
    net::{TcpListener, TcpStream, UdpSocket},
};

pub fn receiver_main() -> std::io::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", 0))?;
    let listener_port = listener.local_addr().unwrap().port();

    connect_to_sender(listener_port);

    for stream in listener.incoming() {
        handle_client(stream.unwrap());
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

fn handle_client(mut stream: TcpStream) {
    let mut buffer = String::new();
    let bytes_read = stream.read_to_string(&mut buffer).unwrap();

    println!("bytes read: {bytes_read}");
    println!("Buffer currently holds: {buffer}");
}
