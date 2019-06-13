use std::io;

fn main(){
    println!("Guess the number");
    println!("Please input your guess:");
    //::是调用String的关联函数new，相当于new是String的静态函数
    let mut guess = String::new();
    //如果没有第一行的use引用，也可以写std::io::stdin().xxx
    io::stdin().read_line(&mut guess)
            .expect("Filed to read line!");
    println!("your guess {}", guess);
}

