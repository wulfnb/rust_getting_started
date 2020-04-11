/*
Primitive Types
Integer: u8, i8 u16, i16, u32, i32, u64, i64, u128, i128 (Number fof bits)
Floats: f32, f64
Boolean: bool
Charaters: char
Tuples
Arrays
*/

pub fn run() {

    // Default is i32
    let a = 5;

    // Default is f64
    let b = 3.5;

    // Explicit type
    let c: i64 = 9843754938;
    println!("{:?}", (a, b, c));

    // find maximum size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // boolean
    let is_active = true;

    // get boolean from expression
    let is_greater = 10 > 15;

    println!("{:?}", (is_active, is_greater));

    // char 
    let x = 'a';
    // imogi unicode
    let face = '\u{1F600}';
    println!("{:?}", (x, face));

}