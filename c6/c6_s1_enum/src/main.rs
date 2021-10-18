enum IpAddrKind {
    v4,
    v6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message::call");
    }
}

fn route(ip_kind: IpAddrKind) {}

fn main() {
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;
    route(four);
    route(six);
    let home = IpAddr {
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::v6,
        address: String::from("::1"),
    };

    let m = Message::Write(String::from("Hello"));
    m.call();
}
