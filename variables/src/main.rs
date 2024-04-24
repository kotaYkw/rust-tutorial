fn main() {
    // 変数
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 定数
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // シャドーイング
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    // スカラー型
    // 整数型
    let x: i32 = -42;
    println!("The value of x is: {}", x);
    // 浮動小数点型
    let x = 2.0; // f64
    println!("The value of x is: {}", x);
    let y: f32 = 3.0; // f32
    println!("The value of y is: {}", y);
    // 足し算
    let sum = 5 + 10;
    println!("The value of sum is: {}", sum);
    // 引き算
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {}", difference);
    // 掛け算
    let product = 4 * 30;
    println!("The value of product is: {}", product);
    // 割り算
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {}", quotient);
    let floored = 2 / 3;
    println!("The value of floored is: {}", floored);
    // 余り
    let remainder = 43 % 5;
    println!("The value of remainder is: {}", remainder);
    // 論理値
    let t = true;
    println!("The value of t is: {}", t);
    let f: bool = false;
    println!("The value of f is: {}", f);
    // 文字型
    let c = 'z';
    println!("The value of c is: {}", c);
    let z = 'ℤ';
    println!("The value of z is: {}", z);
    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    // 複合型
    // タプル型
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);
    // 配列型
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a[0] is: {}", a[0]);
    let a = [3; 5];
    println!("The value of a[0] is: {}", a[0]);
    
}
