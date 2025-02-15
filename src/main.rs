use std::env;
use igrep::Config;

fn main() {
    let args : Vec<String> = env::args().collect();

    let config = Config::from(&args).unwrap();

    println!("{:?}", config);
}
