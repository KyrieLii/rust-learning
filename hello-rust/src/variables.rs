pub fn vars() {
    let x = 5;
    let x = x + 1;
    {
        let x = x*2;
        println!("{x}")
    }
    println!("outer {x}")
}
