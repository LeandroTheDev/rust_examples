use std::io::stdin;

fn main() {
    //Variable declaration
    let numbers = vec![5, 6, 9, 12, 25]; //This is the numbers that the machine will need to calculate
    let mut sum = String::new();
    println!(
        "Enter a number to be found in a sum between the numbers in the array: {:?}",
        numbers
    );
    //Input
    let _result = stdin().read_line(&mut sum).expect("An error occurred.");
    let sum = sum.trim().parse::<i32>().unwrap();
    //Calculation
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            //Check if found
            if numbers[i] + numbers[j] == sum && i != j {
                println!("The sum of number {:?} in array {i} and the number {:?} in array {j} is equals {sum}", numbers[i], numbers[j]);
                return;
            }
        }
    }
    println!("No sum with the number {sum} found");
}
