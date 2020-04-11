pub fn run() {
    // print to console
    println!("Hello from printrs");

    // Basic forrmating
    println!("Number {}", 1);

    // Basic formatting
    println!("New {} is {}","Name", "Wolf");

    // Positional params
    println!("New {0} is {1}","Name", "Wolf");
    
    // Named Arguments
    println!("New {name} is {name2}",name = "Name", name2 = "Wolf");

    // Placeholder traits
    println!("Binary: {:b} Hex {:x} Octal {:o}", 10, 10, 10);

    // Pleace holder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10)

}