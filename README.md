# Rust-Learning

## Cargo
在工作区根目录创建 Cargo.toml
```toml
[workspace]
resolver = "3"
members = [
    "guessing_game",
    "hello_cargo",
    "variables",
]
```
即可管理子package  
在工作区目录通过 `cargo new` 命令创建新 package 时，似乎会自动向工作区目录的 Cargo.toml 中添加 package