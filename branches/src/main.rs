fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // 没有隐式转换，必须是bool类型
    // if number {

    let condition = true;
    // 有点像三元运算符
    let number = if condition { 5 } else { 6 };
    println!("number: {number}");

    // loop 无限循环
    loop {
        println!("again!");
        break;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // 可以从 loop 返回值
        }
    };
    println!("The result is {}", result);

    let mut count = 0;
    // 带标签 loop
    // 用来跳出多层循环
    // 注意标签必须以 ' 开头
    'outer: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'outer;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // while 循环
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for 循环，常用于遍历集合
    let arr = [10, 20, 30, 40, 50];
    for element in arr {
        println!("the value is: {element}");
    }

    // 重写上面while循环版的倒计时
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
