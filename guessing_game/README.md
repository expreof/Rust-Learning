# 猜数字程序

## prelude
rust 会自动导入一组东西到程序中（比如 String），这样开发者就可以不用每次导入
没有自动导入的东西就需要手动导入，用 `use` 导入想要的名字

## 变量
```Rust
let apples = 5; // immutable
let mut bananas = 5; // mutable
```
`Rust` 中用 `let` 创建变量（`C++: auto? auto!`）  
与 `C++` 不同的一点是，变量默认是不可变的（`C++: const? constexpr? consteval?`）  
想要可变的变量需要在变量名称前面加 `mut`

创建一个同名变量，会覆盖（shadowing）之前的变量名

## 标准输入
- `io::stdin()` 返回一个标准输入对象
- `read_line()` 读取用户输入并将其**追加**到参数
- `&` 指示参数是个引用，`mut` 指示参数可变
    > 哦，🐂的，函数声明的时候写一遍引用和可变性还不够，调用的时候也得写
- `read_line` 返回的是 `Result` 类型的对象，是一个 *enum*，跟 `C++` 的不是一回事，更像 `std::variant`。`Result` 有两个可选的值：`Ok` or `Err`，`Ok` 表示成功，包含成功生成的值，`Err` 表示失败，包含错误信息
- `expect` 是 `Result` 的方法，如果 `Result` 是 `Ok`，则返回 `Ok` 持有的值，如果是 `Err`，则使程序崩溃并显示传给它的参数

## 类型推断
🐂的，`parse` 的返回类型竟然是通过给 `guess` 指定类型来确定，来一点小小的 `Rust` 震撼

## match 表达式
`match` 用来模式匹配，没有 `fallthrough`，无需 `break`，不仅限于整型，真是好用  
这里出现的 `break` 和 `continue` 用来影响外层循环

## loop
用来创建一个无限循环

## 依赖
    [dependencies]
    rand = "0.8.5"
向项目添加依赖，等号左边包名，右边指定版本，然后 cargo 处理剩下的事  
`"0.8.5"` 是个简写，完整版是 `^0.8.5`，表示大于等于 `0.8.5` 但不到 `0.9.0`  
*Cargo.lock* 文件保证构建可复现，它确保你在 build 时不会莫名升到 `0.8.6`  
`> cargo update` 用户明确指定升级包版本，此时就无视上面那条啦