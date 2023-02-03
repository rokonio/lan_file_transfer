use receiver::receiver_main;
use sender::sender_main;

mod header;
mod receiver;
mod sender;

fn main() -> std::io::Result<()> {
    let end = std::env::args().nth(1).unwrap();
    match &end[..] {
        "s" | "send" | "sender" => {
            sender_main();
            Ok(())
        }
        "r" | "receive" | "receiver" => receiver_main(),
        o => {
            eprintln!("Not a valid option: {o}");
            Ok(())
        }
    }
}
