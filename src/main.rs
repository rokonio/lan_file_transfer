mod receiver;
mod sender;

pub mod consts {
    pub const SPEPARATOR: &str = "<SEPARATOR>";
    pub const BUFFER_SIZE: usize = 4096;
}

fn main() -> std::io::Result<()> {
    let sender = std::env::args().nth(1).unwrap() == "s";
    if sender {
        sender::sender_main();
        Ok(())
    } else {
        receiver::receiver_main()
    }
}
