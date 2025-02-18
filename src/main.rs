use std::env;
use igrep::{Config, ErrType};

fn main() {
    let args : Vec<String> = env::args().collect();
    let mut result_list: Vec<Result<String, ErrType>> = vec![];
    let mut config = Config::new();
    match Config::from(&args) {
        Ok (x) => config = x,
        Err(x) => result_list.push(Err(x)),
    };
    
    for i in config.file.iter() {
        result_list.push(
            todo!("A nice look for help");
            todo!("A jackass error handling");
            match config.run(i) {
                (_, Some(string)) => {
                    Ok(string)
                },
                (errtype, None) => Err(errtype),
            }
        );
    }
}
