// Primitive: str = immutable fixed-length string
// String = Growable, heap allocated data struture 

pub fn run() {

    let hello = String::from("Hello");

    // get length
    println!("Length {}", hello.len());

    let mut hello2 = String::from("Hello");
    // Push Carecter
    hello2.push('w');
    hello2.push_str(" World");
    println!("{}", hello2);

    // Capacity
    println!("capacity = {}", hello2.capacity());

    // is empty
    println!("Is empty = {}", hello2.is_empty());

    // contains
    println!("If Contains orld = {}", hello2.contains("orl"));

    // replace 
    println!("Replace {}", hello2.replace("World", "Universe"));

    // Loop through string by whitespace
    for word in hello2.split_whitespace(){
        println!("Split word is = {}", word);
    }

    // String with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // assertation
    assert_eq!(2,s.len());
    assert_eq!(10,s.capacity());

}