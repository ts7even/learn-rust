// Primitive str = Immutable fixed-legnth string somewhere in memory, what we have been using so far
// String = Growable, heab-allocated data stucture - Use when you need to modify or own string data. 
// let mut hello - are for single stings "a" not "ba". Called a char

pub fn run() {
    let hello = "Hello"; //Primative String 
    let hello2 = String::from("Hello"); // Growable String

    println!("Length: {}", hello.len());

    

    println!("{}", hello);
    println!();
    println!("Length: {}", hello2.len());
    println!("{}", hello2);

    
}