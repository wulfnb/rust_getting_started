use std::env;
pub fn run(){

    let args: Vec<String> = env::args().collect();

    let command = args[1].clone();

    println!("Args {:?}", args);
    println!("Command {}", command);
    // cargo run name=123
}
