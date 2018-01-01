#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 0,
    };

    println!("user1: {:?}", user1);

    // For this to work user1 has to be mutable
    user1.email = String::from("somebody@example.com");
    println!("user1.email: {}", user1.email);

    let user2 = build_user(user1.email, String::from("username"));
    println!("user2: {:?}", user2);

    let user3 = build_from_user(String::from("a@a.com"), String::from("a"), user2);
    println!("user3: {:?}", user3);
}

fn build_from_user(email: String, username: String, tpl: User) -> User {
    User {
        email,
        username,
        ..tpl // this consumes the instance of User
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}