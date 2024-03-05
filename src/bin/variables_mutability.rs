const MAX_POINTS: u32 = 100_000; // const is always immutable
fn main() {
    let mut x: i32 = 5; // mut means mutable
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    //----------------shadowing-------------------
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value y out of scope is: {y}");

    //---------------shadowing can change the type------------
    let space = "   ";
    let space = space.len();
    println!("The value of space is: {space}");
}
