use std::env;
use igrep::{Config, ErrType, ResultHandle};

fn main() {
    let args : Vec<String> = env::args().collect();
    let mut result_list: Vec<Result<String, ErrType>> = vec![];
    let mut config = Config::new();
    match Config::from(&args) {
        Ok (x) => config = x,
        Err(x) => result_list.push(Err(x)),
    };
    
    for i in config.file.iter() {
        if config.help_is_on() {
            println!("\x1b[1mHelp needed!\x1b[0m");
            config.print_help();
            panic!();
        } else {
            result_list.push(
                match config.run(i) {
                    (_, Some(string)) => {
                        Ok(string)
                    },
                    (errtype, None) => Err(errtype),
                }
            );
        }
    }

    result_list.handle(&config.file);
}


