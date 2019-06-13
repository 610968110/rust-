//需要先在Cargo.toml中添加随机数生成的依赖rand = "0.3.14"，因为Rust标准库中没有这个功能
use rand::Rng;
use std::io;
//猜猜看小游戏
pub fn run() {
    println!("hello world");
    println!("Guess the number");
    //生成随机数，1到100
    let random_num = rand::thread_rng().gen_range(1, 101);
    println!("the random number is {}", random_num);
    println!("Please input your guess:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Filed to read line!");
    println!("your guess {}", guess);
}

