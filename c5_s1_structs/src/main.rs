struct User {
    username: String,
    email:    String,
    sign_in_count:  u64,
    active:   bool,
}

// cannot be compiled for lack of lifetime parameters
// struct UserWithRef {
//     username: &str,
//     email: &str,
// }

fn main() {
    let mut user1 = User {
        email: String::from("Helloworld@rust.com"),
        username: String::from("Hello"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);
    user1.email = String::from("23333@rust.com");
    println!("{}", user1.email);

    let user2 = build_user(String::from("User2@rust.com"), 
                           String::from("User2"));
    println!("{}", user2.email);
    println!("{}", user2.username);

    let user3 = User {
        ..user2
    };

    println!("user3 copied from user2: {}", user3.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}