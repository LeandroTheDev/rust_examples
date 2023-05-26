use std::io;

//Convert String to Int
fn convert_to_int(data_input: &String) -> i32 {
    //Transforming String to int
    let x: i32 = data_input.trim().parse::<i32>().unwrap();
    //Return
    x
}

fn main() {
    let mut sum: i32 = 0;
    let mut value: String = String::new();
    //Input
    println!("Type 2 digits number");
    io::stdin()
        .read_line(&mut value)
        .expect("Error wrong number");
    //Conversion
    let mut value_i32: i32 = convert_to_int(&value);
    
    while value_i32 != 0 {
        //Returns the most Left side number
        let r: i32 = value_i32 % 10;
        //Make the sum
        sum = sum + r;
        //Excludes the most Left Side number
        value_i32 = value_i32 / 10;
    }
    println!("The sum of the two digit numbers is {}", sum);
}
