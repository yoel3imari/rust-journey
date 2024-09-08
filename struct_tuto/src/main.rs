#[derive(Debug)]
struct User {
    username: String,
    is_active: bool,
    id: u64
}

enum Message {
    Text(Option<String>),
    Sender(User)
}

impl User {
    fn username(&self) -> &str {
        &self.username
    }

    fn say_hello() {
        println!("Hello");
    }
}

fn main() {
    let user = User{
        username: String::from("youssef"),
        is_active: true,
        id: 22
    };

    println!("user: {user:?}");
}
