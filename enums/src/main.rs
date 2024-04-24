// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

enum Message {
    Quit,
    Move { x: i32, u: i32 },
    White(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Hello");
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Allaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quater from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    // let _home = IpAddr::V4(12, 0, 0, 1);
    // let _loopback = IpAddr::V6(String::from("::1"));

    let m = Message::White(String::from("Hello"));
    m.call();
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; // no implementation for `i8 + std::option::Option<i8>`

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(1) => println!("One"),
        Some(3) => println!("Three"),
        Some(5) => println!("Five"),
        Some(7) => println!("Seven"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    }

    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quater from {:?}!", state);
    } else {
        count += 1;
    }
}
