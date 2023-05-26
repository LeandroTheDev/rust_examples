use std::io;

//Convert String to Int
fn convert_to_int(data_input: &String) -> i32 {
    //Transforming String to int
    let x: i32 = data_input.trim().parse::<i32>().unwrap();
    //Return
    x
}

fn main() {
    let mut factorial: String = String::new();
    //Input
    io::stdin()
        .read_line(&mut factorial)
        .expect("Wrong factorial");
    println("Type a number lower than 12:");
    //Index
    let mut index: i32 = 1;
    while convert_to_int(&factorial) > 1 {
        //Index Multiply
        index = index * convert_to_int(&factorial);
        //Convert to String
        factorial = (convert_to_int(&factorial) - 1).to_string();
    }
}
