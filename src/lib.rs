use std::path::Path;

fn exists(path: impl AsRef<Path>) -> bool {
    Path::new(path.as_ref()).exists()
}

#[derive(Debug)]
pub struct Config<'a> {
    // current_dir : &'a str,
    query: Query,
    search_string: &'a str,
    file: &'a str,
}

#[derive(Debug, PartialEq)]
struct Query {
    help: bool,
    read: bool,
    search: bool,
    find: bool,
    verbose: bool,
    quiet: bool,
    case_sensitive: bool,
}

impl<'a> Config<'a> {
    fn new() -> Self {
        Self {
            // current_dir : "",
            query: Query::new(),
            search_string: "",
            file: "",
        }
    }
    pub fn from(args: &'a Vec<String>) -> Result<Self, ErrType<'a>> {
        let mut config: Config<'a> = Self::new();
        let mut result : Option<ErrType> = None;

        for i in 1..args.len() {
            config.push(&args[i]);
            ////////////
        } 

        if !result.is_none(){
            return Err(result.unwrap());
        }
        if config.query == Query::new() {
            return Err(NoArgs);
        } 

        Ok(config)
    }

    fn push(&mut self, arguement: &'a String) -> Option<ErrType> {
        let query = &mut self.query;
        let mut args = vec![];
        match arguement.as_str() {
            "--help"            => query.search = true,
            "--search"          => query.search = true,
            "--case_sensitive"  => query.case_sensitive = true,
            "--find"            => query.find = true,
            "--verbose"         => query.verbose = true,
            "--quiet"           => query.quiet = true,
            "--read"            => query.read = true,
            x             => {
                query.help = false;
                args.push(x);
                if x.contains("--") {
                    return Some(UnknownArgs(x));
                } else if exists(x) && self.file.is_empty() {
                    self.file = x;
                } else if self.search_string.is_empty() {
                    self.search_string = x;
                } else {
                    query.help = true;
                    return Some(TooManyArgs(args));
                }
            }
        }
        None
    }
}

impl Query {
    fn new() -> Self {
        Self {
            help: true,
            read: false,
            search: false,
            find: false,
            verbose: false,
            quiet: false,
            case_sensitive: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ErrType<'a> {
    TooManyArgs (Vec<&'a str>),
    NoArgs,
    UnknownArgs(&'a str),
}
use ErrType::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn just_new() {
        let config = Config::new();
        assert!(config.query.help);
        assert!(!config.query.read);
        assert!(!config.query.search);
        assert!(!config.query.find);
        assert!(!config.query.verbose);
        assert!(!config.query.quiet);
        assert!(!config.query.case_sensitive);
    }

    #[test]
    #[should_panic]
    fn config_from_no_args() {
        let arg_vec = Vec::new();
        if let Err(NoArgs) = Config::from(&arg_vec) {
            panic!();
        }
    }

    #[test]
    fn config_from_too_many_args() {
        let arg_vec = vec![
            String::from("/home/.config/something"),
            String::from("--search"),
            String::from("--find"),
        ];
        if let Err(TooMany) = Config::from(&arg_vec) {
            panic!();
        }
    }

    #[test]
    fn config_from_smallest_valid_arguement_list() {
        let arg_vec = vec![
            String::from("/home/.config/something"),
            String::from("some.txt"),
        ];

        let config = Config::from(&arg_vec).unwrap();
        assert_eq!(config.file, "some.txt"); // fails if there's no such file
        assert_eq!(config.search_string, ""); // becomes some.txt if there's no such file
        assert!(!config.query.help);
    }
}
