fn main() {
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // 常量，也是不可变的，类似于 C++ 中的 编译期求值
    // 要求必须指明类型
    // const ONE_MINUTE_IN_SECONDS = 60;
    const ONE_MINUTE_IN_SECONDS: u32 = 60;
    const ONE_HOUR_IN_MINUTES: u32 = 60;
    const ONE_HOUR_IN_SECONDS: u32 = ONE_HOUR_IN_MINUTES * ONE_MINUTE_IN_SECONDS;
    const THREE_HOURS_IN_SECONDS: u32 = ONE_HOUR_IN_SECONDS * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    let res = 5 / 2;
    println!("5/2 = {res}"); // 5/2 = 2
    let res: f64 = 5 as f64 / 2.0;
    println!("5/2.0 = {res}"); // 5/2 = 2.5

    // char 长度为 4 字节
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c: {c}, z: {z}, heart_eyed_cat: {heart_eyed_cat}");

    // tuple
    let tup: (i32, f64, u8) = (500, 3.14, 1);
    let (x, y, z) = tup; // 解包，C++ 的结构化绑定？
    println!("x: {x}, y: {y}, z: {z}");
    let tup0 = tup.0;
    let tup1 = tup.1;
    let tup2 = tup.2;
    println!("tup.0: {tup0}, tup.1: {tup1}, tup.2: {tup2}");

    // 数组
    let arr = [1, 2, 20, 4, 8]; // 自动推导类型和长度
    println!(
        "the max value of arr is: {}",
        arr.iter().max().expect("failed to get max value")
    );

    let arr: [u32; 7] = [1, 2, 3, 4, 5, 6, 7]; // 指定类型和长度
    let first = arr[0];
    println!("first element of arr is: {first}");

    let arr = [3; 5]; // 5 个元素，每个元素的值都是 3
    // 数组越界，编译器会报错
    // 如果在 index 是运行时才确定的，能过编译，但是 index 越界会 panic，即会有越界检查
    // println!("{}", arr[5]);
    println!("{}", arr[4]);
}
