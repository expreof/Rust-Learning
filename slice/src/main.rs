fn main() {
    let s = String::from("hello world!");
    let hello = &s[0..5]; // 引用了，但没完全引，只引了一部分
    let world = &s[6..12]; // 左闭右开
    println!("{hello} {world}");

    let start = &s[..5]; // 从0开始
    let end = &s[6..]; // 到结尾
    let whole = &s[..]; // 整个字符串
    println!("{start} {end} {whole}");

    // Note: slice 的索引必须要位于有效的 UTF-8 字符边界内
    let s = "你好世界";
    let 你 = &s[..3];
    // let bad = &s[0..4]; // 会 panic
    // println!("{你} {bad}");
    println!("{你}");

    let s = String::from("hello world!");
    let word = first_world(&s); // 也接受一个 String 的引用
    // s.clear();
    println!("{word}");
}

// fn first_world(s: &String) -> &str {
fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
