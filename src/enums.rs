
enum Movement {
    Up,
    Down,
    Left,
    Right
}


fn move_it(m: Movement){
    match m {
        Movement::Up => println!("Moving Up"),
        Movement::Down => println!("Moving Down"),
        Movement::Left => println!("Moving Left"),
        Movement::Right => println!("Moving Right")
    }
}

pub fn run(){
    let curser1 = Movement::Left;
    let curser2 = Movement::Right;
    let curser3 = Movement::Up;
    let curser4 = Movement::Down;

    move_it(curser1);
    move_it(curser2);
    move_it(curser3);
    move_it(curser4);
}
