use std::fs::{File, OpenOptions};
use std::io::prelude::*;

fn main() {
    //Creating File
    File::create("test.txt").expect("Cannot create the file \"test.txt\"");
    //Reading the file
    let mut file = OpenOptions::new().write(true).open("test.txt").unwrap();
    //Writing the file
    file.write_all(b"You are reading a created file by rust")
        .expect("Not possible to write in file \"test.txt\"");
    //Output the file
    let mut content = File::open("test.txt").expect("Error reading the file");
    let mut read = String::new();
    content.read_to_string(&mut read);
    println!("{:?}", read);
}
