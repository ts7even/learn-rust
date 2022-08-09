/* 
Primative Types ---
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory, larger the number the more of bits)
FLoats: f32, f64
Boolean (bool)
Characters (char) - 1 character not a string
Tuples - lists 
Arrays - fixed length
Vectors - growing length
Rust is a Statis type language, 
*/

// Rust is  astatically typed language, which means that it must know the types of all variables at compiple time. 
// Howerverm the compiler can usally infer what type we want to use base on the value and how we use it. 


pub fn run(){
    //Default is "i32"
    let x = 1;

    // Default is "F64"
    let y = 2.5;

    // Add explicit type
    let t: i64 = 454545454545;


    // Boolean
    let is_active = true;

    // Get Boolean from expression
    let is_greater: bool = 10 > 5;

    // unicode character with single quotes char. has to be 1 character
    let a1 = 'a';
    let face = '\u{1f600}';

    println!("{:?}", (x,y,t, is_active, is_greater, a1, face));

    // FInd max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
}

