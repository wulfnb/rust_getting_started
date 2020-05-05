pub fn run(){
    let x = 3.0;
    // let y = 2.0;
    let y = 0.0;

    let result:Option<f64> = if y != 0.0 { Some(x/y) } else {None};
    println!("{:?}", result);

    match result
    {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("Cannot Divide {} by {}", x,y)
    }

    if let Some(z) = result { println!("z = {}",z) }
}
