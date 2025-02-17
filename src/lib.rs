use std::{fs, path::Path, io::{ErrorKind, Error}};
// use termion::terminal_size;
use crossterm::terminal::size;



fn file_exists(path: impl AsRef<Path>) -> bool {
    Path::new(path.as_ref()).is_file()
}

#[derive(Debug)]
pub struct Config<'a> {
    // current_dir : &'a str,
    query: Query,
    pub search_string: Vec<&'a str>,
    pub file: Vec<&'a str>,
}

#[derive(Debug, PartialEq)]
struct Query {
    help: bool,
    read: bool,
    search: bool,
    file: bool,
    verbose: bool,
    quiet: bool,
    case_sensitive: bool,
}

impl<'a> Config<'a> {
    pub fn new() -> Self {
        Self {
            // current_dir : "",
            query: Query::new(),
            search_string: vec![],
            file: vec![],
        }
    }
    pub fn from(args: &'a Vec<String>) -> Result<Self, ErrType<'a>> {
        let mut config: Config<'a> = Self::new();

        for i in 1..args.len() {
            match config.push(&args[i]) {
                Nothing => continue,
                x => return Err(x),
            }
        }
        if config.query == Query::new() {
            return Err(NoArgs);
        }
        Ok(config)
    }

    fn push(&mut self, arguement: &'a String) -> ErrType<'a> {
        let query = &mut self.query;

        match arguement.as_str() {
            "--help" => query.search = true,
            "--case_sensitive" => query.case_sensitive = true,
            "--verbose" => query.verbose = true,
            "--quiet" => query.quiet = true,
            "--read" => query.read = true,
            "--file" => {
                query.file = true;
                query.search = false;
                self.file.clear();
            }
            "--search" => {
                query.search = true;
                query.file = false;
                self.search_string.clear();
            }
            x => {
                if let Err(x) = self.parse_str(x) {
                    return x;
                };
            }
        }
        Nothing
    }

    fn parse_str(&mut self, str: &'a str) -> Result<(), ErrType<'a>> {
        let query = &mut self.query;
        query.help = false;
        if str.contains("--") {
            return Err(UnknownArgs(str));
        } else if query.file == !query.search {
            if query.file {
                self.file.push(str);
            } else if query.search {
                self.search_string.push(str);
            } else {
                panic!("bro wtf");
            }
        } else {
            self.file.push(str);
            self.search_string.push(str);
        }
        self.query.file = true;
        Ok(())
    }

    pub fn run(&'a self, address: &'a str) -> (ErrType<'a>, Option<String>) {
        let search_is_on: bool = !self.search_string.is_empty();
        match fs::read(address) {
            Ok(_) => {
                (Nothing, Some(
                if self.query.read && search_is_on {
                    read_and_search(address, &self.search_string)
                } else if search_is_on {
                    search_file(address, &self.search_string)
                } else if self.query.read {
                    read_file(address)
                } else {
                    locate_file(address)
                }))
            }, Err(x) => {
                (ErrType::from(x, address), None)
            }
        }
        
    }
}

fn search_file <'a> (address : &'a str, search_vector : &Vec<&'a str>) -> String {
    let mut searched = String::new();
    match fs::read_to_string(address) {
        Ok(file) => {
            for line in file.lines() {
                let mut line = line.to_string();
                for element in search_vector {
                    line.search_and_highlight(element);
                }
                searched.push_str(&line);
                searched.push('\n');
            }
            searched
        }, _ => panic!("search_file(&str, &Vec<&str>) fucking panics")
    }
}

fn locate_file (address : &str) -> String {
    if file_exists(address) {
        format!("exists! at address : {address}")
    } else {
        panic!("locate_file(&str) fucking panics!")
    }
}

fn read_and_search <'a> (address : &'a str, search_vector : &'a Vec<&'a str>) -> String {
    match fs::read_to_string(address) {
        Ok(mut string) => {
            for element in search_vector {
                string.search_and_highlight(element);
            }
            string
        }, _ => panic!("read_and_search(&str, &Vec<&str> fucking panics"),
    }
}

fn read_file <'a> (address: &'a str) -> String {
    match fs::read_to_string(address) {
        Ok(string) => string,
        _ => panic!("read_file(&str) fucking panics"),
    } 
}

impl Query {
    fn new() -> Self {
        Self {
            help: true,
            read: false,
            search: false,
            file: false,
            verbose: false,
            quiet: false,
            case_sensitive: false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrType<'a> {
    Nothing,
    TooManyArgs,
    NoArgs,
    UnknownArgs(&'a str),
    FileNotFound(&'a str),
    PermissionDenied,
    FileInUse(&'a str),
    UnknownErr,
}
use ErrType::*;

impl <'a> ErrType<'a> {
    fn from (err : Error, address : &'a str) -> Self {
        match err.kind () {
            ErrorKind::AddrInUse => FileInUse(address),
            ErrorKind::NotFound => FileNotFound(address),
            ErrorKind::PermissionDenied => PermissionDenied,
            _ => UnknownErr,
        }
    }
}

#[cfg(test)]
mod config_test {
    use super::*;

    #[test]
    fn just_new() {
        let config = Config::new();
        assert!(config.query.help);
        assert!(!config.query.read);
        assert!(!config.query.search);
        assert!(!config.query.verbose);
        assert!(!config.query.quiet);
        assert!(!config.query.case_sensitive);
        assert!(!config.query.file);
    }

    #[test]
    #[should_panic]
    fn config_from_no_args() {
        let arg_vec = Vec::new();
        if let Err(x) = Config::from(&arg_vec) {
            if x == NoArgs {
                panic!();
            }
        }
    }

    #[test]
    fn config_from_many_but_valid_args() {
        let arg_vec = vec![
            String::from("/home/.config/something"),
            String::from("--search"),
            String::from("--read"),
            String::from("some"),
            String::from("apple"),
            String::from("oranges"),
            String::from("--file"),
            String::from("file.txt"),
            String::from("something/else.txt"),
            String::from("--verbose"),
            String::from("--case_sensitive"),
        ];
        let config = Config::from(&arg_vec).unwrap();
        assert_eq!(config.file, vec!["file.txt", "something/else.txt"]);
        assert_eq!(config.search_string, vec!["some", "apple", "oranges"]);
        assert!(config.query.file);
        assert!(config.query.read);
        assert!(config.query.verbose);
        assert!(config.query.case_sensitive);
        assert!(!config.query.search);
    }

    #[test]
    fn config_from_smallest_valid_arguement_list() {
        let arg_vec = vec![
            String::from("/home/.config/something"),
            String::from("some.txt"),
        ];

        let config = Config::from(&arg_vec).unwrap();
        assert_eq!(config.file, vec!["some.txt"]); // fails if there's no such file
        assert_eq!(config.search_string, vec!["some.txt"]); // becomes some.txt if there's no such file
        assert!(!config.query.help);
    }

    #[test]
    #[ignore]
    fn config_from_average_valid_arguement_list() {
        let _arg_vec = vec![
            String::from("home/.config/something"),
            String::from("oreo.txt"),
            String::from("apple"),
            String::from("--search"),
            String::from("orange"),
        ];
    }
}

trait Search {
    fn search_and_highlight(&mut self, search_element: &str);
}

impl Search for String {
    fn search_and_highlight(&mut self, element: &str){
        let mut colorised_string = String::new();
        let string = self.to_ascii_lowercase();

        let mut start_point = 0;
        while let Some(mut pos) = string[start_point..].find(element.to_lowercase().as_str()) {
            pos += start_point;
            colorised_string.push_str(&self[start_point..pos]);
            colorised_string.push_str("\x1b[1;33m");
            colorised_string.push_str(&self[pos..(pos + element.len())]);
            colorised_string.push_str("\x1b[0;0m");
            start_point = pos + element.len();
        }
        colorised_string.push_str(&self[start_point..]);
        *self  = colorised_string;
    }
}

