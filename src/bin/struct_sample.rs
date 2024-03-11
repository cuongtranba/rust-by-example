#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct RectangleWithFunc {
    width: u32,
    height: u32,
}

impl RectangleWithFunc {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &RectangleWithFunc) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 10,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );

    let rect2 = Rectangle {
        width: 2,
        height: 3,
    };
    println!("The area of the rectangle is {:?} square pixels.", rect2);

    let rec3 = RectangleWithFunc {
        width: 2,
        height: 3,
    };
    println!(
        "The area of the rectangle is {0} square pixels.",
        rec3.area()
    );

    let rect1 = RectangleWithFunc {
        width: 30,
        height: 50,
    };
    let rect2 = RectangleWithFunc {
        width: 10,
        height: 40,
    };
    let rect3 = RectangleWithFunc {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn area(rec: &Rectangle) -> u32 {
    rec.height * rec.width
}
