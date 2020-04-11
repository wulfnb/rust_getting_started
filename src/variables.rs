pub fn run() {

    let name = "Wulf";

    // Mutable
    let mut age = 25;

    println!("My name is {} ang age is {}", name, age);
    age = 26;
    println!("My name is {} ang age is {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID is {}", ID);

    // Assign multipple variables
    let (my_name, my_age) = ("Wulf", 25);

    println!(" {} {} ", my_name, my_age);
}