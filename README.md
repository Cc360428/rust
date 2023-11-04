## 配置源
- `cd ～.cargo`
- `touch config`
- vim config
    ```toml
    [source.crates-io]
    replace-with = 'rsproxy-sparse'
    [source.rsproxy]
    registry = "https://rsproxy.cn/crates.io-index"
    [source.rsproxy-sparse]
    registry = "sparse+https://rsproxy.cn/index/"
    [registries.rsproxy]
    index = "https://rsproxy.cn/crates.io-index"
    [net]
    git-fetch-with-cli = true
    ```
## 基本命令
- rustc
  - `rustc --version`
  - `rustup update stable`
  - `rustc main.rs`
- cargo
  - `cargo --version`
  - ``cargo run``
  - `cargo check`
  - `cargo build`
  - `cargo build --release` 发布版本
  - `cargo update`
  - `cargo tree`
  - `cargo doc`

[//]: # (  - `cargo publish`)
