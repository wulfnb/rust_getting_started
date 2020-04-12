// No tunery if in rust
pub fn run(){
    let age = 18;
    let skip_check = true;

    // if else
    if age >= 32 || skip_check {
        println!("Greater than 32");
    }
    else if age >= 18{
        println!("18 Above");
    }
    else {
        println!("Under aged");
    }

    // short if 
    let is_age_18 = if age >=18 { true } else { false };
    println!("{}", is_age_18);    

}