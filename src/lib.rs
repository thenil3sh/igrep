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
    search_string: Vec<&'a str>,
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
        let mut push_to  = 0;
        for i in 1..args.len() {
            match config.push(&args[i], &mut push_to) {
                Nothing => continue,
                x => return Err(x),
            }
        }
        if config.query == Query::new() {
            return Err(NoArgs);
        }
        Ok(config)
    }

    fn push(&mut self, arguement: &'a String, push_to : &mut u8) -> ErrType<'a> {
        let query = &mut self.query;

        match arguement.as_str() {
            "--help" => query.help = true,
            "--case_sensitive" => query.case_sensitive = true,
            "--verbose" => query.verbose = true,
            "--quiet" => query.quiet = true,
            "--read" => query.read = true,
            "--file" => {
                query.file = true;
                if *push_to == 0 {
                    self.file.clear();
                }
                *push_to = 2;
            }
            "--search" => {
                query.search = true;
                if *push_to == 0 {
                    self.search_string.clear();
                    
                } 
                *push_to = 1;
            }
            x => {
                if let Err(x) = self.parse_str(x, *push_to) {
                    return x;
                };
            }
        }
        Nothing
    }

    fn parse_str(&mut self, str: &'a str, push_to : u8) -> Result<(), ErrType<'a>> {
        let query = &mut self.query;
        if str.contains("--") {
            return Err(UnknownArgs(str));
        } else {
            if matches!(push_to, 0 | 2) {
                self.file.push(str);
                query.file = true;
            }
            if matches!(push_to, 0 | 1) {
                self.search_string.push(str);
                query.search = true;
            }
        }
        // self.query.file = true;
        Ok(())
    }

    pub fn run(&'a self, address: &'a str) -> (ErrType<'a>, Option<String>) {
        match fs::read(address) {
            Ok(_) => {
                (Nothing, Some(
                if self.reading_is_on() && self.search_is_on() {
                    read_and_search(address, &self.search_string)
                } else if self.search_is_on() {
                    search_file(address, &self.search_string)
                } else if self.reading_is_on() {
                    read_file(address)
                } else {
                    locate_file(address)
                }))
            }, Err(x) => {
                (ErrType::from(x, address), None)
            }
        }
        
    }

    pub fn search_is_on(&self) -> bool {
        println!("search is on");
        return !self.search_string.is_empty();
    }

    pub fn file_is_on(&self) -> bool {
        println!("file is on");
        self.query.file
    }

    pub fn reading_is_on(&self) -> bool {
        println!("reading is on");
        self.query.read
    }

    pub fn help_is_on(&self) -> bool {
        println!("help is on");
        self.query.help
    }

    pub fn print_help(&self) {
        panic!();
    }
}

fn search_file <'a> (address : &'a str, search_vector : &Vec<&'a str>) -> String {
    let mut searched = String::new();
    match fs::read_to_string(address) {
        Ok(file) => {
            for (line_number, line) in file.lines().enumerate() {
                let mut line = line.to_string();
                let mut element_found = false;
                for element in search_vector {
                    if line.contains(element) {
                        element_found = true;
                        line.search_and_highlight(element);
                    }
                }
                if element_found {
                    let str = format!("{line_number} | {line}\n");
                    searched.push_str(&str);
                }
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
            help: false,
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
    NeedHelp,
    PermissionDenied(&'a str),
    FileInUse(&'a str),
    UnknownErr,
}
use ErrType::*;

impl <'a> ErrType<'a> {
    fn from (err : Error, address : &'a str) -> Self {
        match err.kind () {
            ErrorKind::AddrInUse => FileInUse(address),
            ErrorKind::NotFound => FileNotFound(address),
            ErrorKind::PermissionDenied => PermissionDenied(address),
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

pub trait ResultHandle {
    fn handle(&self, config : &[&str]);
}

impl <'a> ResultHandle for Vec<Result<String, ErrType<'a>>> {
    fn handle(& self, file_arr : &[&str]) {
        let mut err_count : u32 = 0;
        for (file_count, result) in self.iter().enumerate() {
            match result {
                Ok(string) => {
                    print_heading(file_count, file_arr[file_count]);
                    println!("{string}");
                } _ => err_count += 1,
            }
        }

        if err_count == 0 {
            std::process::exit(0);
        } else if err_count == 1 {
            print!("\x1b[31m[ERROR]\x1b[0m ");
        } else if err_count > 1 {
            println!("\x1b[31mFollowing errors were encountered :\x1b[0m");
        }
        for (file_count, result) in self.iter().enumerate() {
            match result {
                Err(err_type) => print_error(file_count,err_type),
                _ => {},
            }
        }
        std::process::exit(1);
    }
}

fn print_heading (file_number : usize, address : &str) {
    let (width, _) = size().unwrap();
    let mut heading = format!("=====[ FILE {file_number} : {address} ]");
    for _i in heading.len()..(width as usize) {
        heading.push('=');
    }
    println!("{heading}");
}

fn print_error(file_count : usize, result : &ErrType) {
    match result {
        &NoArgs => println!("Expected arguements, nothing was given"),
        &TooManyArgs => println!("Too many arguements"),
        &UnknownArgs(arg) => println!("Unknown arguement : '{arg}'"),
        &FileNotFound(add) => println!("\x1b[33m[FILE : {file_count}] \x1b[0m\x1b[0m{add} : file doesn't exist"),
        &PermissionDenied(add) => println!("\x1b[33m[FILE : {file_count}] \x1b[0m\x1b[0m{add} : permission denied"),
        &FileInUse(add) => println!("\x1b[33m[FILE : {file_count}] \x1b[0m\x1b[0m{add} : file is already in use"),
        _ => println!("\x1b[33m[FILE : {file_count}] \x1b[0m\x1b[0mUnknown error occured"),
    }
}