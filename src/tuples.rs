// grouped values of different types
// max 12 elements
pub fn run() {
    let person: (&str, &str, i8) = ("First", "Second", 25);
    println!("{} {} {}", person.1, person.2, person.0);
}