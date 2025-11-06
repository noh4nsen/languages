use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr)
}

enum Message {
    Quit,
    Move {x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

fn main() {
    let home: IpAddrKind = IpAddrKind
                            ::V4(127, 0, 0, 1);
    let loopback: IpAddrKind = IpAddrKind
                                ::V6(String::from("::1"));
    let localhost_v4 = IpAddr
                                ::V4(Ipv4Addr::new(127, 0, 0, 1));
    let localhost_v6 = IpAddr
                                ::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    dbg!(home);
    dbg!(loopback);
    dbg!(localhost_v4);
    dbg!(localhost_v6);

    Message
        ::Quit.call();
    Message
        ::Move{ x: 32, y: 32 }.call();
    Message
        ::Write(String::from("Teste")).call();
    Message
        ::ChangeColor(10, 100, 96).call();

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');
    
    let abseant_number: Option<i32> = None;
    
}
