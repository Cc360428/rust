#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    // 整型
    let num1: i32 = 42; // 有符号整型
    let num2: u32 = 100; // 无符号整型

    // 浮点型
    let float_num: f64 = 3.14;

    // 布尔型
    let is_true: bool = true;

    // 字符型
    let letter: char = 'A';

    // 字符串
    let static_str: &str = "Hello, Rust!";
    let dynamic_str: String = String::from("Hello");

    // 元组
    let tuple: (i32, f64, char) = (42, 3.14, 'A');

    // 数组
    let array: [i32; 3] = [1, 2, 3];
    let slice: &[i32] = &array[1..3]; // 切片

    // 结构体
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // 枚举
    let move_direction = Direction::Up;

    // 打印所有类型的值
    println!("整型: num1 = {}, num2 = {}", num1, num2);
    println!("浮点型: {}", float_num);
    println!("布尔型: {}", is_true);
    println!("字符型: {}", letter);
    println!("静态字符串: {}", static_str);
    println!("动态字符串: {}", dynamic_str);
    println!("元组: {:?}", tuple);
    println!("数组: {:?}, 切片: {:?}", array, slice);
    println!("结构体: {:?}", person);
    println!("枚举: {:?}", move_direction);
}