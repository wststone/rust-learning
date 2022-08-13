use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io; // prelude // trait

fn main() {
    println!("Hello, world!");
    let secret_num: u32 = thread_rng().gen_range(1..100);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        println!("你猜的数是： {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("小"),
            Ordering::Greater => println!("大"),
            Ordering::Equal => {
                println!("正好");
                break;
            }
        }
    }
}
