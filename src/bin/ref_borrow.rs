fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn calculate_len(s: &String) -> usize {
    return s.len();
}

fn change(s: &mut String) {
    s.push_str(", world")
}
