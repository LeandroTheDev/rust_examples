use std::io;

//Convert String to Int
fn convert_to_int(data_input: &String) -> i32 {
    //Transforming String to int
    let x: i32 = data_input.trim().parse::<i32>().unwrap();
    //Return
    x
}

fn main() {
    //Variable Declaration
    let mut number1: String = String::new();
    println!("Type a number");
    //Terminal Input
    io::stdin().read_line(&mut number1).expect("Wrong number");
    //Variable Declaration
    let mut number2: String = String::new();
    println!("Type another number");
    //Terminal Input
    io::stdin().read_line(&mut number2).expect("Wrong number");

    if convert_to_int(&number1) > convert_to_int(&number2) {
        println!("The number {} is bigger than {}", number1, number2);
    } else {
        println!("The number {} is smaller or equals {}", number1, number2);
    }
}
