use std::env;
use igrep::Config;

fn main() {
    let args = env::args().collect();

    println!("{:?}", Config::from(&args));
}
