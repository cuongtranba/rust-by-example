use std::io;

fn main() {
    let guess = "42".parse::<u16>().expect("Not a number!");
    println!("The value of guess is: {guess}");

    let mytup: (String, i16) = (String::from("string"), 1);

    println!("value mytup: {0}, {1}", mytup.0, mytup.1);

    let a = [1, 2, 3, 4, 5];
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");

    let index = index.trim().parse::<usize>().expect("not a number");

    let element = a[index];
    println!("the value of the element at index {index} is : {element}")
}
