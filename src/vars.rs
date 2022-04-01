// Variable hold primitive data or references to data
// Varables are immutable by default (Cannot reassign them) just like const in javascript
// To make them reuseable, you have to do 'let mut'
// There are explict const varibles that should not change
// Rust is a block-scoped language (Set a varible in a fucntion, only used in that funciton)


pub fn run() {
    let name = "Brad";
    let mut age = 37;

    println!("My name is {} and I am {}", name, age);
    age = 38;
    println!("My name is {} and I am {}", name, age);

    // Define const
    const ID: i32 = 001;
    println!("ID: {}", ID);
    
    // Assign multiple variables
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
    println!();
}