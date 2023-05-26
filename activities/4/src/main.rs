use std::io;

//Convert String to Int
fn convert_to_int(data_input: &String) -> i32 {
    //Transforming String to int
    let x: i32 = data_input.trim().parse::<i32>().unwrap();
    //Return
    x
}

fn main() {
    let mut quantity: String = String::new();
    println!("Type the student test quantity:");
    //Input
    io::stdin().read_line(&mut quantity).expect("Wrong Number");
    let selected_quantity: i32 = convert_to_int(&quantity);
    let mut sum: i32 = 0;

    while convert_to_int(&mut quantity) > 0 {
        let mut grade: String = String::new();
        println!("Type the student test grade:");
        //Input
        io::stdin().read_line(&mut grade).expect("Wrong Number");
        //Add in sums
        sum += convert_to_int(&mut grade);
        //Lower the quantity
        quantity = (convert_to_int(&quantity) - 1).to_string();
    }

    //Calculate the averages
    sum = sum / selected_quantity;

    //Check if passed
    if sum >= 7 {
        println!("The student have passed! with {}", sum);
    } else {
        println!("The student failed! with {}", sum);
    }
}
