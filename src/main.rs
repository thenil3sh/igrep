use std::env;
use igrep::{Config, ErrType};

fn main() {
    let args : Vec<String> = env::args().collect();
    let mut error_list = vec![];
    let mut config = Config::new();
    match Config::from(&args) {
        Ok (x) => config = x,
        Err(x) => error_list.push(x),
    };
    
    for i in config.file.iter() {
        error_list.push(
            match config.run(i) {
                (_, Some(string)) => {
                    if config.search_string.is_empty() {

                    } else {
                        
                    }
                    ErrType::Nothing
                },
                (err, None) => err,
            } 
        );
    }
}
