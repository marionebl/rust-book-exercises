#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

#[derive(Debug)]
struct Color(i8, i8, i8);

#[derive(Debug)]
struct Point(i32, i32, i32);

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

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black: {:?}", black);
    println!("origin: {:?}", origin);
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