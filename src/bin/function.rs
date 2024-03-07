fn main() {
    let mut value = samele();
    println!("{value}");

    value = samele_expression();
    println!("{value}");
}
// return type
fn samele() -> i32 {
    let v = 3;
    return v;
}

fn samele_with_params(v: i32) -> i32 {
    1
}

fn samele_expression() -> i32 {
    let y = {
        let x = 9;
        x
    };
    return y;
}
