use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number !");
    // 变量 默认不可修改
    let min = 0; 
    let max = 1000;
    // mut 表示可修改
    let mut rng = rand::thread_rng();
    // 生成随机数
    let secret_number = rng.gen_range(min..=max);
    // 定义一个计数器
    let mut count = 0;

    // {} 占位符
    println!(
        "Pls input your guess ({} ~ {}) !",
        min, max,
    );

    // 循环体
    loop {

        // String::new() 返回Stirng的一个空实例，:: 类似类的静态方法
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess) // 接收用户输入 赋值给 guess, '&' 是引用
        .expect("Faild to read line"); // 错误处理方法

        // 把输入转化为u32整型
        // parse 方法返回一个 Result类型（枚举），配合match语句
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        count = count + 1;
        println!("you guessed: {}, times: {}", guess, count);

        // match 分支判断
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Equal => {
                println!("Got it");
                break;
            },
            Ordering::Greater => println!("Too Big"),
        }
    }
}
