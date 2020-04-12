// Resizable arrays

pub fn start () {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    println!("{:?}", numbers.len());

    // add value to vector
    numbers.push(7);
    numbers.pop();
    println!("{:?}", numbers);

    // loop
    for x in numbers.iter(){
        println!("{}", x);
    }

    // Loop & mutate 
    for x in numbers.iter_mut(){
        *x *= 3;
    }
    println!("{:?}", numbers);

}