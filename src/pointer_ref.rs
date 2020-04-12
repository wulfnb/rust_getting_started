pub fn run(){
    // primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;
    println!("{:?}", (arr1, arr2));

    // non-primitives if we assign to another variable the first variable will no longer hold the value
    // Need to user a refreence (&) to point to the resourse

    // Vector
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("{:?}", (&vec1, vec2));

}
