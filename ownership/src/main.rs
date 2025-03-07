fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    // s1 此时已经被认为无效
    // println!("{}, world!", s1);
    println!("{}, world!", s2);

    let mut s3 = String::from("WORLD");
    // 原内存会被释放
    s3 = String::from("world");
    println!("hello, {}!", s2);

    let s4 = String::from("hello");
    let s5 = s4.clone(); // 深拷贝
    println!("s4={},s5={}", s4, s5);

    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
    // println!("{s}"); // ... 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
    // 但 i32 是 Copy 的，
    // 所以在后面可继续使用 x

    let s6 = String::from("hello");
    let len = calculate_length(&s6);
    println!("The length of '{}' is {}.", s6, len); // s6 仍然有效

    let mut s7 = String::from("hello");
    change(&mut s7);
    println!("{s7}");

    // 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    let mut s8 = String::from("hello");
    let r1 = &s8;
    let r2 = &s8;
    // let r3 = &mut s8;
    // print!("{r1}, {r2}, {r3}");
    // 引用的作用域从声明的地方开始一直持续到最后一次使用为止
    println!("{r1}, {r2}");
    // 此后 r1 和 r2 不再使用
    let r3 = &mut s8; // OK
    println!("{r3}");
} // 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
// 没有特殊之处

fn change(s: &mut String) {
    s.push_str(", world");
}

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{some_string}");
} // 这里，some_string 移出作用域并调用 `drop` 方法。
// 占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{some_integer}");
} // 这里，some_integer 移出作用域。没有特殊之处

// s 是一个引用，没有所有权
fn calculate_length(s: &String) -> usize {
    s.len()
}
