// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// let home = IpAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// };

// let loopback = IpAddr {
//     kind: IpAddrKind::V6,
//     address: String::from("::1"),
// };

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

fn main() {
    // enum IpAddr {
    //     V4(String),
    //     V6(String),
    // }

    // let _home = IpAddr::V4(String::from("127.0.0.1"));
    // let _loopback = IpAddr::V6(String::from("::1"));

    enum IpAddr {
        V4(u8,u8,u8,u8),
        V6(String),
    }

    let _home = IpAddr::V4(127,0,0,1);
    let _loopback = IpAddr::V6(String::from("::1"));
    
    let m = Message::Write(String::from("hello"));
    m.call();    
}
