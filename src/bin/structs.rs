struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let user1 = User {
        active: true,
        username: String::from("cuong"),
        email: String::from("email"),
        sign_in_count: 3,
    };

    let user2 = User {
        username: String::from("cuong2"),
        ..user1
    };

    println!("{0}", user2.email)
}
