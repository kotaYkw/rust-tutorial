fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    another_function(5);
    print_labeled_measurement(42, 'm');

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    println!("The value of five() is: {}", five());

    println!("The value of pluse_one(5) is: {}", plus_one(5));
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The value of measurement is: {}{}", value, unit_label);
}
