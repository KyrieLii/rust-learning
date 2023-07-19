use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number !");
    // var immutable default
    let min = 0; 
    let max = 1000;
    // mut: mutable
    let mut rng = rand::thread_rng();
    // generate random number
    let secret_number = rng.gen_range(min..=max);
    let mut count = 0;

    // {} : placeholder
    println!(
        "Pls input your guess ({} ~ {}) !",
        min, max,
    );

    // infinite loop
    loop {

        //  create a empty instance of a String.
        // ::new is an associated function of String
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess) // get input of user, & means reference.
        .expect("Faild to read line");

        // trim(): eliminate any whitespace at the beginning and end of String
        // parse(): converts a string to another type
        // `:u32` : which type do we want to trans
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        count = count + 1;
        println!("you guessed: {}, times: {}", guess, count);

        // match (switch)
        match guess.cmp(&secret_number) {
            // case 1
            Ordering::Less => println!("Too Small"),
            // case 2
            Ordering::Equal => {
                println!("Got it");
                break;
            },
            // case 3
            Ordering::Greater => println!("Too Big"),
        }
    }
}
