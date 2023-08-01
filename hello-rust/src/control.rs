pub fn control() {
    println!("==== control =====");

    // if
    let n = 3;
    if n == 0 {
        println!(" == 0");
    } else if n < 5 {
        println!("< 5");
    } else {
        println!(">= 5");
    }

    // loop
    // loop{
    //     println!("looop");
    // }
    let mut count = 1;
    let res = loop{
        count += 1;
        if count == 5 {
            break count * 2; // return this
        }
    };
    println!("{}, {}", count, res);

    // while
    let mut c = 1;
    while c < 10 {
        c+=1;
        println!("{c}")
    }

    // for
    let arr = [1,2,3,4,5];
    for el in arr {
        println!("arr: {el}")
    }
    for el in (1..4).rev() {
        println!("{el}")
    }
}