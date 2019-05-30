#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColour(i32, i32, i32),
}

impl Message {
    fn call(&self) -> String {
        match &self {
            Message::Quit => "Quit".to_string(),
            Message::Move { x, y } => format!("Moved to x: {} y: {}", x, y),
            Message::Write(text) => format!("Wrote {}", text),
            Message::ChangeColour(r, g, b) => format!("Changed colour to {}{}{}", r, g, b),
        }
    }
}

fn main() {
    println!("IP Addresses");
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6("::1".to_string());

    println!("{:?}", home);
    println!("{:?}", loopback);

    println!("Messages");
    let quitter = Message::Quit;
    let mover = Message::Move { x: 20, y: 30 };
    let writer = Message::Write("It's on the wall".to_string());
    let chameleon = Message::ChangeColour(255, 128, 0);
    println!("{}", quitter.call());
    println!("{}", mover.call());
    println!("{}", writer.call());
    println!("{}", chameleon.call());

    println!("{}", Some(5) + 8);
}
