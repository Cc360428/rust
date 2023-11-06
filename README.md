# Rust

## 系统环境配置

- `vim /etc/profile` 追加

```shell
export CARGO_HOME=$HOME/.cargo
export RUSTUP_HOME=$HOME/.rustup
export RUSTUP_DIST_SERVER=http://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=http://mirrors.ustc.edu.cn/rust-static/rustup
export PATH="$HOME/.cargo/bin:$PATH"
```

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
