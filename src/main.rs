use receiver::receiver_main;
use sender::sender_main;

mod header;
mod receiver;
mod sender;
mod utils;

fn main() -> anyhow::Result<()> {
    let end = std::env::args().nth(1).unwrap();
    match &end[..] {
        "s" | "send" | "sender" => {
            sender_main();
            Ok(())
        }
        "r" | "receive" | "receiver" => receiver_main(),
        other => {
            eprintln!("Not a valid option: {other}");
            Ok(())
        }
    }
}
