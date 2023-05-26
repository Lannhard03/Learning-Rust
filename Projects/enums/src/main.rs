#[derive(Debug)]
enum IpAddr {
    V6(String),
    V4(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn print(&self) {
        match &self {
            Message::Write(text) => println!("{text}"),
            _ => (),
        }
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.print();

    let s1 = String::from("Hello ");
    let mut s2 = String::from("world!");
    let s3 = s1 + &s2;
    s2 = String::from("universe!");
    println!("{s2}");
    println!("{s3}");
    let s = format!("{}-{}-{}", "wtf", s2, s3);
    println!("{s}");
}
