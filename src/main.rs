fn main() {
    let s = String::from("Hellow");
    take_ownership(s);
    println!("toankt = {s}");

    let x = 5;
    makes_copy(x);
}

fn makes_copy(x: i32) {
    println!("{x}");
}

fn take_ownership(some_string: String) {
    println!("{some_string}");
}
