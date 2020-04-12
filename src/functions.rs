pub fn run() {
    greeting("Hello", "Wulf");
    
    // bind function value to variable
    let get_sum = add(5,5);
    println!("{}",get_sum);

    // closure
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("{}",add_nums(6, 7));

}

fn greeting(greet: &str, name : &str){
    println!("{} {},Good to see you", greet, name);
}

// Returning
fn add(a: i32, b: i32) -> i32 {
    a + b
}