use std::fs::File;
use std::io::Read;
use std::io::Write;

// Read as string
pub fn run2() {
    let mut data = String::new();
    let mut f = File::open("/etc/hosts").expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    println!("{}", data);
}

// Read as vec

pub fn run3() {
    let mut data = Vec::new();
    let mut f = File::open("/etc/hosts").expect("Unable to open file");
    f.read_to_end(&mut data).expect("Unable to read data");
    println!("{}", data.len());
    println!("{:?}", data);
}

// Write file
pub fn run() {
    let data = "Some data!";
    let mut f = File::create("/tmp/foo").expect("Unable to create file");
    f.write_all(data.as_bytes()).expect("Unable to write data");
}