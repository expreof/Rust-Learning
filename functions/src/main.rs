// 函数定义的出现顺序不是很重要

fn main() {
    println!("Hello, world!");
    another_function(5, 's');
    println!("print five() = {}", five());
}

// 带参数函数
fn another_function(x: i32, unit_label: char) {
    println!("The measurement is {x}{unit_label}");
}

// 注意下面没有分号，返回表达式的值
fn five() -> i32 {
    5
}
