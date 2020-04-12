// Structs used to create custom datatypes

// Treaditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct Color2(u8,u8,u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // fullname function
    fn fullname(&self) -> String {
        format!("Mr {} {}",self.first_name, self.last_name)
    }

    // set lastname
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run(){

    let c = Color {
        red: 255,
        green: 0,
        blue: 0
    };
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut a = Color2(255,75,90);
    println!("Color: {} {} {}", a.0, a.1, a.2);

    let mut p = Person::new("Wulf", "NB");
    println!("Person {} {}", p.first_name, p.last_name);
    p.set_last_name("New");
    println!("Person fullname is {}", p.fullname());
    println!("Person {:?} ", p.to_tuple());


}