// Fixed list
pub fn start () {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    
    // mutatable array
    let mut numbers2: [i32; 5] = [1, 2, 3, 4, 5];

    // Reassigning value
    numbers2[2] = 50;
    println!("{:?}", numbers2);

    // First element
    println!("{}", numbers[0]);

    // Arrays are stack allocated
    println!("Array taken {} bytes", std::mem::size_of_val(&numbers));

    // get slices of two elements
    let slice: &[i32] = &numbers[0..2];
    println!("slice: {:?}", slice);
}