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

# 错误排除

1. 遇到带有`RUST_BACKTRACE=1`,`panic`的错误输出

一般原因:无法发送请求

解决方法:检查 URL 的域名/IP 和协议是否正确,DNS 是否正常

> 如果无法解决,可以将环境变量设置为`RUST_BACKTRACE=1`后运行,将输出提交到 issue

2. `reqwest::Error { kind: Decode, source: Error("expected value", line: 1, column: 1) }`

一般原因:无法以处理返回信息为 json

解决方法:检查 URL 的路径是否正确,URL 后面不要带有`/`

> 如果无法解决,可以提交错误的输出到 issue

3. `{"reason": String("___<此处输出可能不同>___"), "success": Bool(false)}`

一般原因:提供的 token 等信息错误

解决方法:检查 token 等信息是否正确

_注意:_

_不要混淆`Api.global_token`与`Server token`_

_第一个是`put`,`drop`这些管理操作使用的 Token,以 `md5` 储存在服务端配置文件中(注意**是 md5 不是明文**)_

_第二个是`status`上传状态使用的,在新建服务器后会生成_

4. 其他错误

上传 issue,包含输出信息

## 构建

1. 安装[Git](https://git-scm.com/)

2. 安装[rustup](https://rustup.rs/)

3. 构建

```
git clone https://github.com/University-Of-Fool/uof-status-client-rust.git && cd uof-status-client-rust/
cargo build --release
```
