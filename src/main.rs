use std::fmt;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        let scale = 2;
        Rectangle {
            width: dbg!(width * scale),
            height: height,
        }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl User {
    pub fn new(active: bool, username: String, email: String, sign_in_count: u64) -> User {
        User {
            active,
            username,
            email,
            sign_in_count,
        }
    }

    pub fn new_active(username: String, email: String) -> User {
        User::new(true, username, email, 1)
    }

    pub fn new_inactive(username: String, email: String) -> User {
        User::new(false, username, email, 1)
    }

    pub fn print(user: &User) {
        println!("{}", user);
    }

    pub fn new_from(user: User, email: String) -> User {
        User { email, ..user }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "username: {}, email: {}, active: {}, sign_in_count: {}",
            self.username, self.email, self.active, self.sign_in_count
        )
    }
}

fn main() {
    let user1 = User::new_active(String::from("cool-user"), String::from("some@cf.ru"));
    User::print(&user1);
    let user2 = User::new_inactive(String::from("some-user"), String::from("some@other.rs"));
    User::print(&user2);
    let user3 = User::new_from(user2, String::from("new-email@shk.pl"));
    User::print(&user3);

    let rect = Rectangle::new(30, 50);
    println!("rect on unique line: {:?}", rect); // works when add #[derive(Debug)] on struct definition
    println!("rect on many lines: {:#?}", rect);
    dbg!(&rect);
    println!("area: {}", rect.area());

    let rect2 = Rectangle::new(20, 45);
    println!("can hold: {}", rect2.can_hold(&rect));
}
