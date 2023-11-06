use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数游戏！");
    let secret_number = rand::thread_rng().gen_range(1..100);
    // println!("神秘数字是：{}", secret_number);

    loop {
        println!("猜一个数字");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("有错误");
                continue;
            }
        };

        println!("你猜测测的数是：{}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too samll"),
            Ordering::Greater => println!("To big"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
}
