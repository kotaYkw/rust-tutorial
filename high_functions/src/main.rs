fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_towice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn returns_closures() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let answer = do_towice(add_one, 5);
    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>();
}
