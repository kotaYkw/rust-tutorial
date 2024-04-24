use std::collections::HashMap;

#[allow(unused)]
fn main() {
    // Vector
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    v.push(6);
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    match v.get(2) {
        Some(third) => println!("Third element is {}", third),
        None => println!("There is no third element"),
    }
    // let does_not_exist = &v[100]; // panic!
    let does_not_exist = v.get(100);
    // let first = &v[0]; // immutable reference
    // v.push(7); // mutable reference
    // println!("The first element is: {}", first);
    for i in &mut v {
        *i += 50;
    }
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ]

    // String
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = String::from("initial contents");
    s.push_str("bar");
    s.push('l');
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    let s  = format!("{}-{}-{}", s1, s2, s3);
    let h = s1[0]; // error: the trait `std::ops::Index<std::ops::Range<usize>>` is not implemented for `String`
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // 所有権がムーブされる
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // 上書き
    scores.entry(String::from("Yellow")).or_insert(50); // なければ挿入
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; //古い値に基づいて更新
    }

}
