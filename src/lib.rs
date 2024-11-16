use std::fs;

pub fn run(config: Config) -> Result<(), String> {
    let file_text = get_file_text(&config.file_name)?;
    let search_results = search(&config.search_str, &file_text, config.match_case);

    print_result(&config.search_str, &search_results);

    if let Some(output_file_name) = config.output_file {
        if let Err(output_error) = save_as_file(&output_file_name, &search_results) {
            return Err(output_error);
        }
    }

    Ok(())
}

fn search(search_str: &str, text: &str, match_case: bool) -> Vec<(usize, String)> {
    let mut result: Vec<(usize, String)> = Vec::new();
    let search_for = if match_case { search_str.to_string() } else { search_str.to_lowercase() };

    for (index, line) in text.lines().enumerate() {
        let search_in = if match_case { line.to_string() } else { line.to_lowercase() };
        if search_in.contains(&search_for) { result.push((index + 1, line.to_string())); }
    }

    result
}

fn get_file_text(file_name: &str) -> Result<String, String> {
    let file = match fs::read_to_string(file_name) {
        Ok(file) => file,
        Err(e) => return Err(format!("Unable to read the file specified: {e}")),
    };

    let file = file.trim_start_matches("\u{feff}").to_string();

    Ok(file)
}

fn print_result(search_str: &str, result: &Vec<(usize, String)>) {
    if result.len() == 0 {
        println!("Provided file doesn't contain the string \"{}\"", search_str);
        return ();
    }

    println!("\"{}\" was found in the next lines:\n", search_str);
    result.iter().for_each(|(index, line)| { println!("[line {}] {}", index, line); });
}

fn save_as_file(output_file_name: &str, result: &Vec<(usize, String)>) -> Result<(), String> {
    let mut output = String::new();

    for (index, line) in result.iter() {
        let line = format!("{index} {line}\n");
        output.push_str(line.as_str());
    }

    match fs::write(output_file_name, output) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Unable to write to file: {err}"))
    }
}

pub struct Config {
    file_name: String,
    search_str: String,
    output_file: Option<String>,
    match_case: bool,
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Config, String> {
        let file_name = Self::get_arg(&args, "file-name", false, false)?;
        let search_str = Self::get_arg(&args, "search-str", false, false)?;
        let match_case = Self::get_arg(&args, "match-case", true, true).is_ok();
        let output_file = Self::get_arg(&args, "output-file", false, true).ok();

        Ok(Config { file_name, search_str, match_case, output_file })
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

    const SAMPLE_TEXT: &str = "\
How in the world
Does that make sense?
I don't know";

    #[test]
    fn successful_search() {
        let search_str = "how";
        let result = tuple_to_lines(search(search_str, SAMPLE_TEXT, false));

        assert_eq!(result, vec!["How in the world"]);
    }

    #[test]
    fn failed_search() {
        let search_str = "whew";
        let result = tuple_to_lines(search(search_str, SAMPLE_TEXT, false));

        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn successful_case_insensitive_search() {
        let search_str = "How";
        let result = tuple_to_lines(search(search_str, SAMPLE_TEXT, true));

        assert_eq!(result, vec!["How in the world"]);
    }

    #[test]
    fn failed_case_insensitive_search() {
        let search_str = "how";
        let result = tuple_to_lines(search(search_str, SAMPLE_TEXT, true));

        assert_eq!(result, Vec::<String>::new());
    }

    fn tuple_to_lines(result: Vec<(usize, String)>) -> Vec<String> {
        result.iter().map(|x| x.1.clone()).collect()
    }
}