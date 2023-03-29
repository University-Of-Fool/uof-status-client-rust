# uof-status-client-rust

一个用 Rust 编写的 [uof-status](https://github.com/University-Of-Fool/uof-status) 客户端。

# 开发进度

## 计划中的功能

- Deadline: **Apr.10th**

* [x] [uof-status-api](https://github.com/University-Of-Fool/uof-status/blob/main/APIREADME.md)
* [x] 解析命令行参数
* [ ] 读取配置文件

## 未来可能加入的功能

- [ ] 并发支持多服务器

## 目前遇到的问题

- [ ] GET 请求无法正常处理 json

## 构建

1. 安装[Git](https://git-scm.com/)

2. 安装[rustup](https://rustup.rs/)

3. 构建

```
git clone https://github.com/University-Of-Fool/uof-status-client-rust.git && cd uof-status-client-rust/
cargo build --release
```
