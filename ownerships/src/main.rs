fn main() {
    // 所有権
    let s1 = gives_ownership(); // gives_ownershipは、戻り値をs1にムーブする
    let s2 = String::from("hello"); // s2がスコープに入る
    let s3 = takes_and_gives_back(s2); // s2はtakes_and_gives_backにムーブされ、戻り値もs3にムーブされる
    println!("s1 = {}, s3 = {}", s1, s3);

    // 借用
    let s4 = String::from("hello");
    let len = calculate_length(&s4);
    println!("The length of '{}' is {}", s4, len);

    // 可変な参照
    let mut s5 = String::from("hello");
    // let r1 = &mut s5; // 可変な参照は一度に一つしか持てない
    change(&mut s5);
    let r1 = &s5; // change関数のスコープが終わったので、参照を作成できる
    let r2 = &s5; // 不変な参照は複数持てる
    // let r3 = &mut s5; // 不変な参照と可変な参照は同時に持てない
    println!("{}, {}", r1, r2);
    println!("{}", s5);

    // 宙に浮いた参照
    let referencu_to_nothing = dangle();
    println!("{}", referencu_to_nothing);

    // スライス
    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    println!("The first word is: {}", word);
    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[..]);
    println!("The first word is: {}", word);
} 

// 所有権を呼び出し元に返す
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

// 所有権を受け取り、所有権を返す
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// 引数を借用する
fn calculate_length(s: &String) -> usize {
    s.len()
}

// 引数を可変な参照として借用する
fn change(some_string: &mut String) {
    some_string.push_str(", world ");
}

// 文字列を直接返すことで、sがスコープを抜けても宙に浮いた参照が作られない
fn dangle() -> String {
    let s = String::from("hello");
    s
}

// 文字列スライスを返す
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}