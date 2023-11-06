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

### rustc

- `rustc --version`
- `rustup update stable`
- `rustc main.rs`

### cargo

- `cargo --version`
- `cargo run`
- `cargo check`
- `cargo build`
- `cargo build --release` 发布版本
- `cargo update`
- `cargo tree`
- `cargo doc`
- `cargo publish` 推送 io 包管理目录

## 基本

### 变量

```rust
fn main() {
    let nub = 5; // 不可变
    println!("{}", nub);
    // nub = 10; // cannot assign twice to immutable variable 不可以变

    let mut nub2 = 10;
    /*
    如果不加着一行会报错 nub2
    = help: maybe it is overwritten before being read?
    = note: `#[warn(unused_assignments)]` on by default
    */
    println!("{}", nub2);
    nub2 = 11;
    println!("{}", nub2)
}
```

### 常量

- 不可以使用 mut，常量永远都是不可变的
- 声明常量使用 const 关键字，它的类型必须被标注
- 常量可以在任何作用域内进行声明，包括全局作用域
- 常量只可以绑定到常量表达式，无法绑定到函数的调用结果或只能在运行时才能计算出的值

```rust
const LANGUAGE: &'static str = "Rust";
const NOT_FOUNT: i32 = 404;
fn main() {
    const DAY: i32 = 1;
    println!("{}", DAY);
    println!("{}", NOT_FOUNT);
    println!("{}", LANGUAGE);
}
```

### 覆盖（Shadowing）

- demo1

```rust
fn main() {
    let x = 8;
    println!("{}", x);
    let x = x + 1;
    println!("{}", x);
    let x = x * 2;
    println!("{}", x);
}
/*
8
9
18
*/
```

- demo2

```rust
fn main() {
    let ss = "   "; // str
    let aa = ss.len(); // usize
    println!("{}", aa)
}
```

### 数据类型

- 整数溢出
  - `u8` 的值时 0-255，如果设置 `u8` 的值设置了 256
    - 在变异模式下 `build` 情况下运行时会 `panic`
    - 在发布模式下 `-- release` 不会`panic` 会执行**环绕**操作;
      - `256=0`
      - `257=1`
- 基础展示

```rust
fn main() {
    // ********************类型********************
    // 常规声明
    let nub: i32 = 5;

    // 后缀声明
    let an_integer = 5i32;

    // 自动推导类型
    let x = 42; // x has type i32
    let y = 1.0; // y has type f64

    println!("整型 {} {} {} {}", nub, an_integer, x, y);

    // ********************bool********************
    let t = true;
    let f: bool = false;
    println!("bool {} {}", t, f);

    // ********************char********************
    let x = 'x';
    let two_hearts = '💕';
    println!("char {} {}", x, two_hearts);
}
```

| Data Type | Description                        | Example                         |
| --------- | ---------------------------------- | ------------------------------- |
| `i8`      | 8-bit signed integer               | `let num: i8 = -5;`             |
| `i16`     | 16-bit signed integer              | `let num: i16 = -200;`          |
| `i32`     | 32-bit signed integer              | `let num: i32 = -2000;`         |
| `i64`     | 64-bit signed integer              | `let num: i64 = -200000;`       |
| `i128`    | 128-bit signed integer             | `let num: i128 = -2000000000;`  |
| `u8`      | 8-bit unsigned integer             | `let num: u8 = 5;`              |
| `u16`     | 16-bit unsigned integer            | `let num: u16 = 200;`           |
| `u32`     | 32-bit unsigned integer            | `let num: u32 = 2000;`          |
| `u64`     | 64-bit unsigned integer            | `let num: u64 = 200000;`        |
| `u128`    | 128-bit unsigned integer           | `let num: u128 = 2000000000;`   |
| `f32`     | 32-bit floating-point              | `let num: f32 = 3.14;`          |
| `f64`     | 64-bit floating-point              | `let num: f64 = 3.14159265359;` |
| `bool`    | Boolean (true or false)            | `let is_true: bool = true;`     |
| `char`    | Unicode character                  | `let ch: char = 'A';`           |
| `str`     | String (slice of UTF-8 characters) | `let s: &str = "Hello, Rust!";` |

- 数字可读性
  - 数字可以加上前缀 0x、0o、0b 分别表示十六进制数、八进制数、二进制数
  - 为了改善数字的可读性，可以在数字类型之间加上下划线(`_`)，比如： `1_000` 等同于 1000， `0.000_001` 等同于 `0.000001`

```rust
fn main() {
    println!("{}", 0xABCDEFu32);
    println!("{}", 0o12345670i32);
    println!("{}", 0b00110011u32);
    println!("{}", 1_000_000u32);
}
/*
11259375
2739128
51
1000000
*/
```

### 数组

#### API

1. **创建数组**：

   - `let arr: [T; N] = [value1, value2, ...];`: 创建一个包含指定类型 `T` 和大小 `N` 的数组，其中元素的值初始化为指定的值。

2. **访问数组元素**：

   - 使用方括号 `[]` 和索引来访问数组元素。例如：`let element = arr[index];`

3. **数组长度**：

   - 数组的长度可以通过 `.len()` 方法获取：`let len = arr.len();`

4. **数组迭代**：

   - 你可以使用 `iter()` 方法来迭代数组的元素：

   ```rust
     for element in arr.iter() {
         // 处理每个元素
     }
   ```

5. **查找元素**：

   - 使用 `.contains()` 方法检查数组是否包含特定元素。返回 `bool` 值。例如：`let contains_value = arr.contains(&value);`

6. **数组切片**：

   - 你可以使用切片来获取数组的一部分元素。例如，获取前两个元素：`let slice = &arr[0..2];`

7. **比较数组**：

   - 使用 `==` 运算符来比较两个数组是否相等。例如：`if arr1 == arr2 { /* 数组相等 */ }`

8. **数组排序**：

   - 你可以使用 `.sort()` 方法对数组进行排序。要求数组的元素类型实现了 `Ord` trait。例如：`arr.sort();`

9. **查找最大值和最小值**：

   - 使用 `.iter().max()` 和 `.iter().min()` 方法来查找数组中的最大值和最小值。

10. **数组转换为向量（`Vec`）**：

    - 使用 `.to_vec()` 方法将数组转换为动态大小的向量。例如：`let vec = arr.to_vec();`

11. **数组转换为切片**：
    - 数组可以通过引用转换为切片：`let slice: &[T] = &arr;`

> 请注意，由于数组的大小是固定的，所以很多向量操作（如添加和删除元素）在数组上不可用。如果你需要更灵活的数据结构，可以考虑使用 Rust 的 `Vec` 类型，它支持动态大小。

#### example

```rust
fn main() {
    // 创建一个包含5个整数的数组
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    for value in numbers.iter() {
        println!("number {}", value)
    }

    println!("第一个值：{}", numbers[0]);
    println!("最后一个值：{}", numbers[numbers.len() - 1]);

    numbers[2] = 100; // 如果修改的话，需要把numbers 的改为 mut 可变即可

    println!("number 2 -> {}", numbers[2]);
    numbers.sort(); //
    println!("Reversed Sorted Numbers {:?}", numbers);
    let value = 202;
    let contains = numbers.contains(&value);
    println!("contains -> {}", contains);
    numbers.sort_by(|a, b| b.cmp(a));
    println!("Reversed Sorted Numbers {:?}", numbers);

    let max_value = numbers.iter().max();
    let mut max: i32 = 0;
    match max_value {
        Some(&max_value) => {
            max = max_value;
            println!("The maximum number in the array is: {}", max_value);
        }
        None => {
            println!("The array is empty.");
        }
    }
    println!("max --> {}", max);
    let numbers1 = [1, 2, 3, 4, 0];
    // 需要数量相等
    if numbers == numbers1 {
        println!("== true")
    } else {
        println!("== false")
    }
    // 转为向量
    let mut numbers1 = numbers1.to_vec();
    numbers1.push(1);

    // 创建一个动态数组
    let mut array_run = vec![1, 2, 3];
    println!("before {}", array_run.len());
    array_run.push(199); // 向向量中添加元素
    println!("after {}", array_run.len());
    for value in array_run.iter() {
        println!("array_run {}", value)
    }
}
```

### 元组（Tuples）

> 元组（Tuple）是 Rust 中的一种复合数据类型，它可以包含不同类型的值，并且是一个不可变的数据结构。元组是有序的，意味着你可以访问和操作元组中的元素，但一旦创建，元组的内容不能被修改

```rust
fn main() {
    let person = ("Cc", 1997_06_12, 168.9);
    println!("{:?}", person);
    println!("{}", person.0);
    println!("{}", person.2);
    // 解构元组
    let (name, age, height) = person;
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Height: {} feet", height);
    // 使用元组作为函数返回值
    println!("{:?}", get_person())
}

fn get_person() -> (String, i32, f64) {
    return ("Cc".to_string(), 11, 9.3);
}
```
