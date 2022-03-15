use std::io;

fn main() {
    println!("Guess the Number !");
    println!("Pls input your guess !");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Faild to read line");

    print!("you guessed: {}", guess);

}
