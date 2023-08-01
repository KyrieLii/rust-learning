pub fn fun() {
	println!("hellow");
	f1(888, "abc");

    let f2v = f2();
	println!("f2(): {f2v}");

    let f3v = f3(f2v);
    println!("f3(): {f3v}");
}

fn f1(x: i32, y: &str) {
	println!("world, {x} {y}");
}

fn f2() -> i32 {
	5
}

fn f3(i: i32) -> i32 {
	i + 1
}