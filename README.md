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

> 栈内存

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

### 切片

> 允许你引用集合（数组、向量、字符串等）的一部分元素而不需要复制整个集合。切片是不可变的，因此不能修改集合的内容，但它们允许你安全地访问集合的子集

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];

    // 创建一个切片，包含数组的前三个元素
    let slice = &numbers[0..3];

    // 访问切片元素
    for number in slice {
        println!("Number: {}", number);
    }

    // 切片也可以使用简化的语法
    let short_slice = &numbers[..2]; // 从开头到索引2（不包括2）
    let rest_slice = &numbers[2..]; // 从索引2到末尾
    println!("{:?} {:?} ", short_slice, rest_slice);
    // 字符串切片
    let text = "Hello, Rust!";
    let text_slice = &text[7..11]; // 创建一个包含 "Rust" 的字符串切片

    println!("Text slice: {}", text_slice);
}
```

### 数组和切片的区别

在 Rust 中，数组（Array）和切片（Slice）是两种不同的数据结构，它们有一些重要的区别：

#### **可变性 1**

- 数组是固定大小的并且不可变的数据结构。一旦创建，数组的大小和内容都不能更改。
- 切片是可变大小的，但是切片本身是不可变的。你可以创建一个新的切片来引用相同的数据，并且可以改变切片的大小，但不能更改切片的内容。

#### **大小**

- 数组具有固定的大小，一旦定义，其大小不能改变。
- 切片可以具有可变的大小，可以根据需要引用不同大小的数据。

#### **内存分配**

- 数组通常在栈上分配内存，因为它们是固定大小的。
- 切片引用的数据可以在栈上或堆上分配，取决于切片引用的数据的生存期和所有权关系。

#### **索引**

- 数组的元素可以通过索引访问，索引从 0 开始。
- 切片的元素也可以通过索引访问，使用与数组相同的语法。

#### **传递和所有权**

- 数组通常是通过值传递的，这意味着在将数组传递给函数时会复制整个数组。
- 切片通常通过引用传递，这意味着在将切片传递给函数时，只传递了对数据的引用，而不复制数据。这提高了性能并减少了内存开销。

#### **可变性 2**

- 数组中的元素不能被更改，因为数组是不可变的。
- 切片可以是不可变的，也可以是可变的。可变切片允许修改引用的数据。
- 下面是一个示例来说明数组和切片之间的差异：

  ```rust
  fn main() {
      // 创建一个数组
      let array = [1, 2, 3, 4, 5];

      // 创建一个切片，引用数组的一部分
      let slice = &array[1..4];

      println!("Array: {:?}", array);
      println!("Slice: {:?}", slice);
  }
  ```

> 注意，切片是对数组的引用，因此在切片中的元素修改会影响原始数组。但是，切片本身是不可变的，因此它的大小和内容不能更改。这是数组和切片之间的一些关键区别。

### 函数

- 语法

```rust
fn function_name(parameter1: Type1, parameter2: Type2) -> ReturnType {
    // 函数体，包含执行的代码
    // 可选的 return 语句用于返回值
    // 最后一个表达式的值将作为返回值
}
```

- 例子

```rust
fn main() {
    add("Cc");
    println!("{}", foo(1));
}

fn add(name: &str) {
    println!("{}", name)
}

fn foo(x: i32) -> i32 {
    x + 1
}
```

### if & for

```rust
fn main() {
    let number = 42;
    let is_rainy = true;
    let option = Some(10);
    let result = 15;

    // if 表达式
    if number < 0 {
        println!("Number is negative");
    } else if number == 0 {
        println!("Number is zero");
    } else {
        println!("Number is positive");
    }

    // if let 表达式（用于模式匹配）
    if let Some(value) = option {
        println!("Option has a value: {}", value);
    } else {
        println!("Option is None");
    }

    // match 表达式（用于更复杂的模式匹配）
    match result {
        1 => {
            println!("Result is 1")
        }
        2 | 3 => println!("Result is 2 or 3"),
        4..=10 => println!("Result is between 4 and 10 (inclusive)"),
        _ => println!("Result is something else"),
    }

    // while 循环
    let mut i = 0;
    while i < 5 {
        println!("While loop: i is {}", i);
        i += 1;
    }

    // loop 循环
    let mut j = 0;
    loop {
        if j >= 5 {
            break;
        }
        println!("Loop: j is {}", j);
        j += 1;
    }

    // for 循环
    let numbers = [1, 2, 3, 4, 5];
    for number in numbers.iter() {
        println!("For loop: Number is {}", number);
    }

    // 条件表达式
    let weather = if is_rainy { "rainy" } else { "sunny" };
    println!("Weather is {} today.", weather);
}
```

## 特性

### 所有权

#### Stack（栈） & Heap（堆）

> Rust 中，Stack（栈）和 Heap（堆）是两个重要的内存区域，它们用于存储不同类型的数据，栈和堆代码在运行时可供使用的内存

- **Stack（栈）**

  - 栈中的数据都必须占用已知固定的大小
  - 以放入值的顺序存储值并以相反顺序取出值
  - 后进先出 （last in ，first out）
  - 增加数据叫做进栈（pushing onto the stack）
  - 移出数据叫做出栈（popping off the stack）

- **Heap（堆）**

  - 在编译时大小未知或大小可能变化的数据，要改为存储堆上
  - 堆是缺乏组织的：当想堆放入数据
  - 内存分配器（memory allocator）在堆的某处找到一块足够大的空位，标记已为使用，并返回一个表示该位未知地址的指针（pointer）
  - 以上这个过程称作在堆上分配内存（allocating on the heap）有时简称为`分配（allocating）`将数据推入栈中并不被认为是分配

- 总结

  - 指向放入堆中的数据是已知并且固定大小的
  - 将指针放到栈上，不过当实际数据是，必须访问指针

- 区别

  - 入栈比在堆上分配内存要快，因为（入栈时）分配器无需为存储新数据去搜索内存空间，其位置总是在栈顶
  - 在堆上分配内存则需要更多的工作，这是因为分配器必须首先找到一块足够存放数据的内存空间，并接着做一些记录为下一次分配做准备
  - 访问堆上的数据比访问栈上的数据要慢，因为通过指针来访问

- 例子

```rust
fn main() {
    // Stack 上的整数
    let x = 5;
    let y = x; // 整数是 Copy 类型，值被复制，不会转移所有权
    println!("x: {}, y: {}", x, y);

    // Heap 上的字符串
    let s1 = String::from("Hello");
    let s2 = s1; // 字符串不是 Copy 类型，所有权被移动
    // 编译错误，s1 不再有效
    // println!("s1: {}", s1);

    // 正确，s2 拥有字符串
    println!("s2: {}", s2);
}
```

> 在上述示例中，我们创建了整数 `x` 和字符串 `s1`。整数是 Copy 类型，因此在将 `x` 的值复制给 `y` 后，`x` 仍然有效。但是，字符串不是 Copy 类型，当将 `s1` 的所有权移交给 `s2` 时，`s1` 不再有效，所有权被转移。这展示了 Stack 和 Heap 上数据的不同行为。
> 要在 Heap 上存储数据，你需要使用 `Box`、`Rc`、`Arc` 等智能指针或内置的数据类型（如 `String` 和 `Vec`），它们负责分配和管理 Heap 上的内存。这允许你在 Rust 中有效地管理动态分配的数据，同时保持内存安全。

#### 规则

- 每个值都有一个变量，这个变量是该值的所有者
- 每个值同时只能右一个所有者
- 当所有者超出作用域（scope）时，该值将被删除

#### 函数所有权

**函数参数和所有权传递**：

- **所有权的移动**：当将具有所有权的值传递给函数时，所有权将从调用方移动到函数内部，调用方将不再拥有该值。

  ```rust
  fn take_ownership(s: String) {
      // s 拥有 String 的所有权
      println!("Value: {}", s);
  }

  fn main() {
      let s1 = String::from("Hello");
      take_ownership(s1); // 所有权被移动到函数
      // 编译错误，s1 不再有效
      // println!("Value: {}", s1);
  }
  ```

- **借用**：函数可以通过借用（引用）来使用值，而不获取其所有权。不可变引用允许多个部分同时访问值，但不能修改它。

  ```rust
  fn borrow(s: &String) {
      // s 是 String 的不可变引用
      println!("Value: {}", s);
  }

  fn main() {
      let s1 = String::from("Hello");
      borrow(&s1); // 借用 s1，不移动所有权
      // 正确，s1 仍然有效
      println!("Value: {}", s1);
  }
  ```

- **可变借用**：使用可变引用可以在函数内修改值。

```rust
fn modify(s: &mut String) {
    s.push_str(", World");
}

fn main() {
    let mut s1 = String::from("Hello");
    modify(&mut s1); // 使用可变引用修改 s1
    println!("Value: {}", s1);
}
```

**函数返回值和所有权传递**：

- **返回值的所有权**：函数可以返回拥有值的所有权，从函数内部移动到调用方。

  ```rust
  fn create_string() -> String {
      let s = String::from("Hello from function");
      s // 所有权被移动到调用方
  }

  fn main() {
      let s1 = create_string(); // 接收返回值的所有权
      println!("Value: {}", s1);
  }
  ```

- **返回引用**：函数也可以返回引用，而不是所有权。

  ```rust
  fn get_length(s: &String) -> usize {
      s.len() // 返回引用
  }

  fn main() {
      let s1 = String::from("Hello");
      let len = get_length(&s1); // 返回引用，不移动所有权
      println!("Length: {}", len);
  }
  ```

### Struct

```rust
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }

    fn get_reference(&self) -> &Person {
        self
    }

    fn update_name(&mut self, new_name: String) {
        self.name = new_name;
    }
    fn say_hello(&self) {
        println!(
            "Hello, my name is {} and I'm {} years old.",
            self.name, self.age
        );
    }
}

struct Rectangle(u32, u32);
impl Rectangle {
    fn area(&self) -> u32 {
        self.0 * self.1
    }
}

fn main() {
    let mut person1 = Person {
        name: String::from("Cc"),
        age: 26,
    };
    println!("{} {}", person1.name, person1.age);
    person1.name = String::from("Holly");
    person1.say_hello();

    let mut super_cc = Person::new(String::from("Super"), 18);
    let reference = &mut super_cc;
    reference.update_name(String::from("SuperCC"));
    super_cc.say_hello();
    let s = super_cc.get_reference();
    s.say_hello();

    let rectangle = Rectangle(5, 10);
    println!("Area: {}", rectangle.area());
}
```

### 枚举

```rust
#[derive(Debug)]
enum TrafficLight {
    Red,
    // Yellow,
    // Green,
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    // Alabama,
    // Alaska,
    NewYork,
}

impl Coin {
    fn value(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(_) => 25,
        }
    }
}

fn main() {
    let red = TrafficLight::Red;
    let coin = Coin::Quarter(UsState::NewYork);
    println!("{:?} {:?}", red, coin);

    match coin {
        Coin::Penny => println!("It's a penny!"),
        Coin::Nickel => println!("It's a nickel!"),
        Coin::Dime => println!("It's a dime!"),
        Coin::Quarter(ref state) => println!("It's a quarter from {:?}", state),
    }

    println!("{}", coin.value());
}
```

```rust
// 定义一个函数，用于执行除法操作，可能返回 Some 包含结果，或者 None 表示除法失败。
fn divide(dividend: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 {
        None // 如果除数为零，返回 None 表示除法失败。
    } else {
        Some(dividend / divisor) // 否则返回 Some 包含除法结果。
    }
}

fn main() {
    // 调用 divide 函数，进行两次除法操作。
    let result1 = divide(10.0, 2.0); // 10.0 / 2.0，成功。
    let result2 = divide(5.0, 0.0); // 5.0 / 0.0，失败，除数为零。

    // 使用 match 表达式来处理 divide 函数的返回值 result1。
    match result1 {
        Some(value) => println!("Result 1: {}", value), // 如果除法成功，打印结果。
        None => println!("Result 1: Division by zero!"), // 如果除法失败，打印错误消息。
    }

    // 使用 match 表达式来处理 divide 函数的返回值 result2。
    match result2 {
        Some(value) => println!("Result 2: {}", value), // 如果除法成功，打印结果。
        None => println!("Result 2: Division by zero!"), // 如果除法失败，打印错误消息。
    }
}
```

### Package/Crate/Module

### package

> 可以自定义包

- Cargo.toml

```toml
[package]
name = "rust_cc"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[lib]
name = "mylib"
path = "src/mylib.rs"

[[bin]]
name = "myapp"
path = "src/main.rs"

[dependencies]
```

- mylib.rs

```rust
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

```

- main.rs

```rust
fn main() {
    let result = mylib::add_numbers(5, 3);
    println!("Result: {}", result);
}
```

#### lib 使用

- test_lib.rs

```rust
// 定义一个函数，将两个数字相加
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

// 定义一个结构体
pub struct Person {
    pub name: String,
    pub age: u32,
}

// 实现结构体的方法
impl Person {
    // 打印个人信息
    pub fn print_info(&self) {
        println!("姓名: {}", self.name);
        println!("年龄: {}", self.age);
    }
}
```

- main.rs

```rust
mod test_lib;

use test_lib::{add_numbers, Person};

fn main() {
    // 使用自定义库中的函数
    let result = add_numbers(5, 3);
    println!("相加结果: {}", result);

    // 使用自定义库中的结构体和方法
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    person.print_info();
}

```

### Crate

### Module

模块（Module）是 Rust 中用于组织和管理代码的一种机制。通过模块，可以将相关的函数、结构体、枚举以及其他项组织在一起，形成一个逻辑上的单元。模块有助于提高代码的可读性、可维护性和重用性。

在 Rust 中，模块可以嵌套，形成层级结构。模块可以包含其他模块，从而创建一个模块树。这种层级结构提供了命名空间的概念，防止命名冲突，并允许在不同的模块中定义同名的项。

模块的定义使用 mod 关键字，后跟模块名称，例如 mod my_module。模块定义通常放在单独的文件中，文件名与模块名称相匹配，并使用 .rs 扩展名

#### super

`super` 关键字在 Rust 中用于访问父模块中的项。它主要用于以下两个方面：

1. 访问父模块中的函数、结构体、常量等：当在子模块中需要访问父模块中定义的项时，可以使用 `super` 关键字。通过 `super` 关键字，我们可以在子模块中引用父模块的项，并使用它们的功能。

2. 解决命名冲突：在 Rust 中，模块可以嵌套，并形成层次结构。在这种情况下，可能会发生命名冲突，即在不同层级的模块中存在相同名称的项。使用 `super` 关键字可以明确指定要访问的是父模块中的项，而不是当前模块或子模块中的同名项。

在使用 `super` 关键字时，需要注意以下几点：

- `super` 关键字只能在模块内部使用，用于指代父模块。

- `super` 关键字后面需要紧跟两个冒号 `::`，用于指定要访问的父模块中的项。

- 可以多次使用 `super` 关键字来访问更高层级的父模块中的项。例如，`super::super::module1::function()` 可以用于访问父模块的父模块中的函数。

- 父模块中的项必须是公共的（使用 `pub` 关键字修饰），以便在子模块中能够访问。

通过使用 `super` 关键字，我们可以在模块层次结构中导航，并在需要时访问父模块中的项。这有助于代码的组织和重用，并解决命名冲突的问题。

#### 例子

- main.rs

```rust
mod module1;
mod module2;

fn main() {
    module1::module1::hello_module1();
    module2::module2::hello_module2();
    let person = module2::module2::Person::new("John", 25);
    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    module2::module2::print_parent_message();
}
```

- module1.rs

```rust
pub mod module1 {
    pub fn hello_module1() {
        println!("Hello from module1!");
    }
}

```

- module1.rs

```rust
pub mod module2 {
    pub fn hello_module2() {
        println!("Hello from module2!");
    }

    // 声明公共结构体
    pub struct Person {
        pub name: String,
        pub age: u32,
    }

    pub mod super_test {
        pub const MESSAGE: &str = "Hello from parent module!";
    }

    pub fn print_parent_message() {
        print!(
            "Message from parent module: {}",
            super::module2::super_test::MESSAGE
        );
    }
    impl Person {
        pub fn new(name: &str, age: u32) -> Person {
            Person {
                name: String::from(name),
                age,
            }
        }
    }
}

```

### use

在 Rust 中，`use`关键字有几种不同的写法，可以根据具体的需求选择适合的形式

1. 导入单个项：

   ```rust
   use path::to::module::Item;
   ```

   > 这种写法将特定的项（函数、结构体、枚举等）从指定的模块路径导入到当前作用域中。可以在后续代码中直接使用导入的项。

2. 导入多个项：

   ```rust
   use path::to::module::{Item1, Item2, Item3};
   ```

   > 这种写法可以同时导入多个项，每个项之间用逗号分隔。

3. 导入整个模块：

   ```rust
   use path::to::module::*;
   ```

   > 这种写法使用通配符`*`导入指定模块中的所有项。需要注意，使用通配符导入会将模块中的所有项都引入到当前作用域，可能会引起命名冲突，因此需要谨慎使用。

4. 使用`as`关键字进行重命名：

   ```rust
   use path::to::module::Item as RenamedItem;
   ```

   > 这种写法将导入的项进行重命名，使其在当前作用域中使用不同的名称。这在解决命名冲突或者简化使用时很有用。

5. 在函数内使用局部作用域：

   ```rust
   fn my_function() {
       use path::to::module::Item;
       // 在函数内部临时导入特定项
   }
   ```

   > 这种写法将`use`语句放在函数内部，仅在该函数内部的局部作用域中有效。这可以避免全局作用域中的命名冲突。

## 集合

### Vector

- 基本数据同类型

```rust
fn main() {
    println!("第一种方式 new");
    let mut vec_new: Vec<i32> = Vec::new();
    vec_new.push(1);
    vec_new.push(34);

    for v_value in vec_new.iter() {
        println!("{} ", v_value)
    }

    match vec_new.get(1) {
        Some(vec_new) => println!("vec_new get index == {}", vec_new),
        None => println!("index error"),
    }

    println!("vec_new len {}", vec_new.len());

    println!("\n第二种方式 宏");
    let mut vec_h = vec![1, 3, 55];
    vec_h.push(120);
    for v_value in vec_h.iter() {
        println!("{} ", v_value)
    }

    println!("{}", vec_h[0]);
    println!("vec_h len {}", vec_h.len());
    println!("");
    vec_new.remove(0);
    vec_h.remove(0);
    println!("vec_new len {}", vec_new.len());
    println!("vec_h len {}", vec_h.len());
    println!("\n遍历方式");
    for element in &vec_new {
        println!("{}", element);
    }

    let mut vec_x = Vec::new();
    vec_x.push("Cc");
    vec_x.push("Holly");
    println!("{:?}", vec_x);
}
```

- 不同类型--枚举

```rust
// #[derive(Debug)]
enum Element {
    Integer(i32),
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let mut v_new: Vec<Element> = Vec::new();
    v_new.push(Element::Integer(10));
    v_new.push(Element::Int(100));
    v_new.push(Element::Float(10.11));
    v_new.push(Element::Text(String::from("Super")));

    for element in &v_new {
        match element {
            Element::Integer(value) => println!("{}", value),
            Element::Int(value) => println!("{}", value),
            Element::Float(value) => println!("{}", value),
            Element::Text(value) => println!("{}", value),
        }
    }

    // println!("{:?}", v_new);
    let v_t = vec![
        Element::Integer(1),
        Element::Int(100),
        Element::Float(10.11),
        Element::Text(String::from("Super")),
    ];

    for element in &v_t {
        match element {
            Element::Integer(value) => println!("{}", value),
            Element::Int(value) => println!("{}", value),
            Element::Float(value) => println!("{}", value),
            Element::Text(value) => println!("{}", value),
        }
    }
    // println!("{:?}", v_t)
}
```

- 在运行（`"{:?}"`）运行失败原因，必须增加`#[derive(Debug)]`原因
  > 在 Rust 中，`#[derive(Debug)]`是一个用于自动生成实现 Debug trait 的属性（Attribute）
  > Debug trait 是一个用于打印调试信息的特征，它提供了一个默认的格式化输出
  > 在你的代码中，通过在 Element 枚举上添加`#[derive(Debug)]`，你告诉编译器自动生成实现 Debug trait 的代码
  > 这样做的好处是，在使用 println!宏打印 v_new 和 v_t 时，可以使用`{:?}`格式化符号来打印整个 Vec 及其中的元素。
  > 如果没有添加`#[derive(Debug)]`，println!宏将无法直接打印 Vec 和枚举类型，因为它们默认情况下没有实现 Debug trait。
  > 通过添加`#[derive(Debug)]`，Rust 编译器会自动为你的类型生成一个合适的调试输出，使得你可以方便地打印和调试你的代码。
  > 需要注意的是，Debug trait 是标准库提供的，它通常用于调试目的。如果你希望自定义打印的格式，可以实现自己的 Debug trait 方法。

### String

### HashMap
