use std::env;
use igrep::Config;

fn main() {
    let args = env::args().collect();

    println!("\n{args:?}\n");
    let address = &args[2];
    
    println!("Mode selected : {mode}");
    println!("File to read : {address}");


    let oreo = fs::read_to_string(address).unwrap();
    println!("{oreo}");
}
