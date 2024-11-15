use std::env;
use std::fs;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args).unwrap();
    let file_text = get_file_text(&config.file_name);
    let file_text = if config.match_case { file_text } else { file_text.to_lowercase() };

    if file_text.contains(&config.search_str()) {
        println!("This file contains the string \"{}\"", config.search_str());
    } else {
        println!("This file doesn't contain the string \"{}\"", config.search_str());
    }
}

fn get_file_text(file_name: &str) -> String {
    let file = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let file = file.trim_start_matches("\u{feff}").to_string();

    file
}

struct Config {
    file_name: String,
    search_str: String,
    match_case: bool,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, String> {
        let file_name = Self::get_arg(&args, "file-name", false, false)?;
        let search_str = Self::get_arg(&args, "search-str", false, false)?;
        let match_case = Self::get_arg(&args, "match-case", true, false).is_ok();

        Ok(Config { file_name, search_str, match_case })
    }

    pub fn search_str(&self) -> String {
        let str = &self.search_str.clone();
        if self.match_case { str.to_string() } else { str.to_lowercase() }
    }

    fn get_arg(args: &Vec<String>, arg_name: &str, is_flag: bool, is_optional: bool) -> Result<String, String> {
        let full_arg_name = String::from("--") + arg_name;
        let arg = args.iter().find(|val| val.starts_with(&full_arg_name));
        let not_found_message = format!("{arg_name} is not provided");
        let parse_error_message = format!("Unable to parse {arg_name}, provide value in format --arg-name=arg-value, e.g. --file-name=superfile.txt");

        let result = match arg {
            Some(arg) => arg.split("=").nth(1),
            None => return if is_optional { Err(String::new()) } else { Err(not_found_message) },
        };

        match result {
            Some(val) => Ok(val.to_string()),
            None => if is_flag { Ok("flag_arg".to_string()) } else { Err(parse_error_message.to_string()) },
        }
    }
}