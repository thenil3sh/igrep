use std::{fs, env};
// use std::thread;
// use std::time;

fn main() {
    let args : Vec<String> = env::args().collect();
    let mode = &args[1];

    println!("\n{args:?}\n");
    let address = &args[2];
    
    println!("Mode selected : {mode}");
    println!("File to read : {address}");


    let oreo = fs::read_to_string(address).unwrap();
    
}
