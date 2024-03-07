fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    // stack only data
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    // Ownership and functions
    let s = String::from("value");
    take_ownership(s);

    make_copy(x);

    println!("form make copy {x}");

    // return value and scope
    let s1 = give_ownership();
    let s2: String = String::from("hello");
    let s3 = take_and_give_back(s2);
    // s1 ok
    // s2 error because it give the ownership to take_and_give_back
    println!("s1 = {s1}, s2 = {s2}, s3 = {s3}")
}

fn take_ownership(v: String) {
    println!("{v}")
}

fn make_copy(v: i32) {
    println!("{v}")
}

fn give_ownership() -> String {
    let s = String::from("yours");
    return s;
}

fn take_and_give_back(v: String) -> String {
    return v;
}
