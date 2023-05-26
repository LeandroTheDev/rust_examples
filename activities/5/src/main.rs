fn main() {
    let string: String = String::from("I am a string with spaces");
    let mut string_without_spaces: String = String::from("");
    for string in string.split_whitespace() {
        string_without_spaces.push_str(string);
    }
    println!("{string_without_spaces}");
}
