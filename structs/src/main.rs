fn main() {
    // 構造体の定義
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user1.email);
    println!("{}", user1.username);
    println!("{}", user1.active);
    println!("{}", user1.sign_in_count);
    user1.email = String::from("anotherone@example.com");
    let user2 = build_user(user1.email, user1.username);
    println!("{}", user2.email);
    let user3 = User {
        email: String::from("anoter@example.com"),
        username: String::from("anotherone"),
        ..user2
    };
    println!("{}", user3.sign_in_count);

    // タプル構造体
    let black = Color(0, 0, 0);
    println!("{}, {}, {}", black.0, black.1, black.2);
    let origin = Point(0, 0, 0);
    println!("{}, {}, {}", origin.0, origin.1, origin.2);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // email: email,
        username, // username: username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);