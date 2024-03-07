fn main() {
    let mut count = 0;
    'couting_up: loop {
        println!("count = {count}");
        let mut remainning = 10;

        loop {
            println!("remainning = {remainning}");
            if remainning == 0 {
                break;
            }
            if count == 2 {
                break 'couting_up;
            }
            remainning -= 1;
        }
        count += 1;
    }
    println!("end count = {count}")
}
