use std::env;
use std::fs;

pub fn run() {
    let file_name = get_arg("file-name", Some("File name is not provided"), false, false)
        .unwrap_or_else(|err| panic!("{err}"));

    let search_str = get_arg("search-str", Some("Search string is not provided"), false, false)
        .unwrap_or_else(|err| panic!("{err}"));

    let match_case = match get_arg("match-case", None, true, true) {
        Ok(_) => true,
        _ => false
    };

    let file_text = get_file_str(&file_name);
    let search_str = if match_case { search_str } else { search_str.to_lowercase() };
    let file_text = if match_case { file_text } else { file_text.to_lowercase() };

    if file_text.contains(&search_str) {
        println!("This file contains the string \"{search_str}\"");
    } else {
        println!("This file doesn't contain the string \"{search_str}\"");
    }
}

fn get_arg(arg_name: &str, panic_message: Option<&str>, is_flag: bool, is_optional: bool) -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    let full_arg_name = String::from("--") + arg_name;
    let arg = args.iter().find(|val| val.starts_with(&full_arg_name));
    let not_found_message = match panic_message {
        Some(msg) => msg.to_string(),
        None => format!("{arg_name} is not provided"),
    };

    let parse_error_message = "Unable to parse {arg_name}, provide value in format --arg-name=arg-value, e.g. --file-name=superfile.txt";

    let result = match arg {
        Some(arg) => arg.split("=").nth(1),
        None => return if is_optional { Err(String::new()) } else { Err(not_found_message) },
    };

    match result {
        Some(val) => Ok(val.to_string()),
        None => if is_flag { Ok("flag_arg".to_string()) } else { Err(parse_error_message.to_string()) },
    }
}

fn get_file_str(file_name: &str) -> String {
    let file = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let file = file.trim_start_matches("\u{feff}").to_string();

    file
}