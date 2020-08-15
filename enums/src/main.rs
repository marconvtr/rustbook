enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String)m
}

enum Message{
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    Changecolor(i32,i32,i32),
}

let home = IpAddr::V4(127,0,0,1);

let loopback = IpAddr::V6(String::from("::1"));

fn main() {

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("Hello, world!");
}
