use std::fs;

pub fn run(config: Config) -> Result<(), String> {
    let file_text = get_file_text(&config.file_name);
    let file_text = if config.match_case { file_text } else { file_text.to_lowercase() };

    match search(&config.search_str(), &file_text) {
        Some(line) => println!("The next line from the text contains \"{}\"\n   -> {}", config.search_str(), line),
        None => println!("Provided file doesn't contain the string \"{}\"", config.search_str())
    }

    Ok(())
}

fn search(search_str: &str, text: &str) -> Option<String> {
    let mut split = text.split("\n");

    match split.find(|str| str.contains(search_str)) {
        Some(line) => Some(line.to_string()),
        None => None,
    }
}

fn get_file_text(file_name: &str) -> String {
    let file = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let file = file.trim_start_matches("\u{feff}").to_string();

    file
}

pub struct Config {
    file_name: String,
    search_str: String,
    match_case: bool,
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Config, String> {
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
        let not_found_message = format!("{arg_name} is not provided");
        let parse_error_message = format!("Unable to parse {arg_name}, provide value in format --arg-name=arg-value, e.g. --file-name=superfile.txt");
        let full_arg_name = format!("--{arg_name}");
        let arg = args.iter().find(|val| val.starts_with(&full_arg_name));

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successful_search() {
        let search_str = "How";
        let text = "\
How in the world
Does that make sense?
I don't know";

        assert_eq!(search(search_str, text), Some("How in the world".to_string()));
    }

    #[test]
    fn failed_search() {
        let search_str = "how";
        let text = "\
How in the world
does it make sense?
I don't know";

        assert_eq!(search(search_str, text), None);
    }
}