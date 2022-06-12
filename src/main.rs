use std::{io::Read, env};

use rb_core::*;

fn buffer_from_file(path: &str) -> Vec<u8> {
    let mut file = std::fs::File::open(path).expect("File not there");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Could not read file");
    // println!("{:?}", buffer);
    buffer
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Hello, world! {:?}", args);
    let bootrombuffer = Some(buffer_from_file(&args[1]));
    let rombuffer = Some(buffer_from_file(&args[2]));


    let cpu = CPU::new(bootrombuffer, rombuffer);
    alliswell();
}

