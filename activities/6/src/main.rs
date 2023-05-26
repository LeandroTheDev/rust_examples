struct Client {
    name: String,
    password: String,
    is_logged: bool,
}
impl Client {
    fn is_logged(&self) {
        if self.is_logged {
            println!("The cliet is logged");
        } else {
            println!("The cliet is not logged");
        }
    }
}
fn main() {
    let client = Client {
        name: String::from("Robert"),
        password: String::from("123"),
        is_logged: false,
    };
    client.is_logged();
}
