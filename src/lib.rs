// use std::env;

#[derive (Debug, PartialEq)]
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
#[derive (Debug, PartialEq)]
pub struct Config<'a> {
    current_dir: &'a str ,
    query: Vec<Querry>,
    string: &'a str,
}

impl <'a> Config <'a> {
    pub fn new() -> Config<'a> {
        Config {
            current_dir: "",
            query: Vec::new(),
            string: "",
        }
    }
}

impl <'a> From<&'a Vec<String>> for Config<'a> {

    fn from(args: &'a Vec<String>) -> Config<'a> {

        let mut config: Config<'a> = Config::new();
        config.current_dir = args[0].as_str();

        for index in 1..args.len() {
            if args[index].contains("--") {
                config.query.push(match args[index].as_str() {
                    "--help" => Help,
                    "--find" => Find,
                    "--read" => Read,
                    "--has" => Has,
                    "--verbose" => Verbose,
                    "--quiet" => Quiet,
                    x => {
                        eprintln!("Unknown arguement '{x}'");
                        std::process::exit(1);
                    }
                });
            } else {
                if config.string == "" {
                    config.string = &args[index];
                } else {
                    eprintln!("Unknown arguement '{}'", args[index]);
                    std::process::exit(1);
                }
            }
        }
        if config.string.is_empty() && config.query.is_empty() {
            config.query.push(Help);
        }
        config
    }
}

#[cfg(test)]
mod test {

    use super::{Config, Querry};

    #[test]
    fn parses_a_normal_arguement() {
        let args = vec![String::from("./"), String::from("--help")];

        assert_eq!(Config{current_dir : "./", query : vec![Querry::Help], string : ""}, Config::from(&args));
    }

    #[test]
    fn parses_without_any_arguements() {
        let args = vec![String::from("Lorem")];
        assert_eq!(Config{
            current_dir : "Lorem",
            query : vec![Querry::Help],
            string : "",
        }, Config::from(&args));
    }

    #[test]
    #[should_panic]
    fn literally_no_arguements_were_passed(){
        assert_eq!(Config::from(&vec![]), Config::new());
    }
}
