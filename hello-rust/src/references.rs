/**
* - At any given time, you can have either one mutable reference or any number of immutable references.
* - References must always be valid.
*/


pub fn refs() {
    let s1 = String::from("hello");

    let len = cal_len(&s1); // &: reference
    // dereferencing *

    println!("len: {}", len);

    // change(&s1);

    let mut ss = String::from("helloooo");
    change2(&mut ss);
}

fn cal_len(s: &String) -> usize {
    s.len()
}

// fn change(ss: &String) {
//     ss.push_str(", world"); // will error!
// }

fn change2(ss: &mut String) {
    ss.push_str(", world");
}