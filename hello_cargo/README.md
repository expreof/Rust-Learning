本目录由 `> cargo new hello_cargo` 创建
Catgo.toml 由 `Cargo` 创建，是 `Cargo` 的配置文件
src 目录和其中的 main.rs 也是由 'Cargo' 创建

## Build

### Default (Dev or Debug)
`> cargo build`
创建可执行文件

### Release
`> cargo build --release`
打开优化，生成的可执行文件运行得更快

## Run
`> cargo run`
编译并运行可执行文件，非常方便

## Check
`cargo check`
检查代码是否可编译，比生成更快，但是不生成二进制文件