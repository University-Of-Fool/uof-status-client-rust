# uof-status-client-rust

一个用 Rust 编写的 [uof-status](https://github.com/University-Of-Fool/uof-status) 客户端。

# 使用方法

可以输入以下命令查看帮助

```
uof-status --help
```

> **警告：URL 最后不要带有`/`**

- 配置文件格式示例:

```
url = "http://127.0.0.1:4044"
server_id = 1
server_token = "dxfhqtzfrrf5fuyc"
time = 60
online = true

```

# 不稳定的功能

- 读取配置文件

- 生成配置文件

- 使用以带有域名的 URL 运行

## 构建

1. 安装[Git](https://git-scm.com/)

2. 安装[rustup](https://rustup.rs/)

3. 构建

```
git clone https://github.com/University-Of-Fool/uof-status-client-rust.git && cd uof-status-client-rust/
cargo build --release
```
