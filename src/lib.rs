use std::env;

enum Querry {
    Find,
    Read,
    Has,
    Verbose,
    Quiet,
    Help,
}
use Querry::*;

// Suggested code may be subject to a license. Learn more: ~LicenseLog:4843641.
struct Config {
    current_dir : 'static &str,
    query : Vec<Querry>,
    string : str,
}

impl Config {
    fn new() -> Config {
        Config {
            current_dir : "",
            query : Vec::new(),
            string : "",
        }
    }
}

impl From<Vec<String>> for Config {
    fn from(args : Vec<String>) -> Config {
        let mut config = Config::new();
        config.current_dir = args[0].clone();

        for word in args {
            if word.contains("--") {
                config.query.push(
                match word.as_str() {
                    "--help" => Help,
                    "--find" => Find,
                    "--read" => Read,
                    "--has" => Has,
                    "--verbose" => Verbose,
                    "--quiet" => Quiet,
                    x => {
                        eprintln!("Unknown querry '{x}'");
                        std::process::exit(1);
                    }
                }
            );
                
            } else {

            }
        } 

        config
    }

}


