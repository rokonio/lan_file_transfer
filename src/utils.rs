use crate::header::Header;
use anyhow::Result;
use std::net::TcpStream;

pub fn recv_header(stream: &mut TcpStream) -> Result<Header> {
    use std::io::Read;
    let mut buf = Vec::with_capacity(Header::SIZE);
    stream
        .by_ref()
        .take(Header::SIZE as u64)
        .read_to_end(&mut buf)?;

    let header = Header::decode(
        buf.try_into()
            .expect("Not enough bytes received to parse a header"),
    )?;
    println!("Received: {header:?}");
    Ok(header)
}

pub fn recv_content(stream: &mut TcpStream, amount: u32) -> std::io::Result<Vec<u8>> {
    use std::io::Read;
    let mut buf = Vec::with_capacity(amount as usize);
    stream.by_ref().take(amount as u64).read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn send_all(
    stream: &mut TcpStream,
    mut header: Header,
    data: Option<&[u8]>,
) -> std::io::Result<()> {
    use std::io::Write;
    let data = if let Some(data) = data {
        if data.len() != header.length as usize {
            eprintln!("Content length differ from header length. This is not fatal but should be reported");
            header.length = data.len() as u32;
        }
        data
    } else {
        &[]
    };
    println!("Sending: {header:?}");
    stream.write_all(&header.encode())?;
    stream.write_all(data)?;
    Ok(())
}
