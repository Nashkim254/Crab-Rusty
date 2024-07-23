use std::net::IpAddr;

enum IPAddrKind {
    IPV4(u8, u8, u8, u8),
    IPV6(String),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn some_fn() {
        println!("Lets code in rust");
    }
}
struct IPAddr {
    kind: IPAddrKind,
    addr: String,
}
pub fn run() {
    println!("val {}", value_in_cents(Coin::Shilling(State::Ke)));
    // let localhost = IPAddrKind::IPV4(127,0,0,1);
}

fn route() -> i8 {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y.unwrap_or(0);
    sum
}

enum Coin {
    Shilling(State),
    Dollar,
    Euro,
    Pound,
}
#[derive(Debug)]
enum State {
    US,
    Ke,
    EU,
    UK,
}
fn value_in_cents(value: Coin) -> u8 {
    match value {
        Coin::Dollar => 1,
        Coin::Shilling(state) => {
            println!("Lucky {:#?} sh", state);
            5
        }
        Coin::Euro => 10,
        Coin::Pound => 15,
    }
}
