// Rust 标准库定义了一组东西，自动导入到每个 Rust 程序中。
// 这个叫做 prelude 的功能包含了很多常用的功能，比如 println! 和 String 类型。
// 这些功能被称为 prelude
// 但是有一些功能不在 prelude 中，比如 I/O
// 这些功能被放在了标准库的不同模块中，需要使用 use 语句导入。
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100); // 生成一个 1 到 100 的随机数, 左闭右闭区间

    loop {
        println!("Please input your guess.");

        // 创建了一个变量，因为需要它可变，所以使用 mut 关键字
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("猜对了！");
                break;
            }
        }
    }
}
