use core::fmt;
use std::error::Error;
use std::num::ParseIntError;
use std::fs;
use std::path::Path;
use regex::Regex;
use std::env;

struct Config {
    path: String,
    nth_number: Option<usize>,
    test_run: bool,
    get_numbers: bool,
    help: bool,
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let position_short = parse_cli_args("-p", &args).ok();
    let position_long = parse_cli_args("--position", &args).ok();
    let config = Config {
        path: args[args.len() - 1].clone(),
        nth_number: if position_short.is_some() { position_short } else { position_long },
        test_run: args.contains(&String::from("-t")) || args.contains(&String::from("--test")),
        get_numbers: args.contains(&String::from("-g")) || args.contains(&String::from("--get_numbers")),
        help: args.contains(&String::from("-h")) || args.contains(&String::from("--help")),
    };

    if config.help {
        print_help_msg();
    }
    else {
        parse_dir(&config);
    }
}

fn print_help_msg() {
    println!("This tool allows you to rename a season or playlist to conform to the \"E01.mkv\" pattern.\n");
    println!("Usage: conformer [OPTIONS] [PATH TO DIR]\n");
    println!("Options:");
    println!("-t,\t--test\t\t\tA testrun without renaming the files");
    println!("-g,\t--get_numbers\t\tShows all found numbers so that you may select the correct index for the \"-p\" argument");
    println!("-p,\t--position <INDEX>\tSelects which numbers to use for the renaming process");
    println!("-h,\t--help\t\t\tShows this message");

}


#[derive(Debug)]
struct InvalidInputError;

impl Error for InvalidInputError {}

impl fmt::Display for InvalidInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You fucked up")
    }
}

fn parse_cli_args(argument: &str, args: &[String]) -> Result<usize, Box<dyn Error>> {
    args[args.iter()
    .position(|x| x == argument).ok_or(InvalidInputError)? + 1]
    .parse().map_err(|e: ParseIntError| e.into())
}

fn parse_dir(config: &Config) {
    match fs::read_dir(Path::new(&config.path)) {
        Ok(entries) => {
            let mut list = entries.map(|entry|
                    entry.unwrap().path().to_str().unwrap().to_string()).collect::<Vec<String>>();
            list.sort();
            for entry in list
            {
                if config.get_numbers {
                    println!("{:?}", find_all_numbers(&entry));
                }
                else {
                    let number = &find_all_numbers(&entry)[config.nth_number.expect("invalid number selected")];
                        if config.test_run {
                        println!("{}",format_title(&get_file_extention(&entry), number, config));
                    }
                    else {
                        rename_file(&entry, &format_title(&get_file_extention(&entry), number, config).to_string());
                    }
                }
            }
        },
        Err(e) => println!("Error: reading dir: {}", e),
    }
}

fn find_all_numbers(filepath: &str) -> Vec<String> {
    let re = Regex::new(r"\d+").unwrap();

    re.find_iter(filepath)
        .map(|m| m.as_str().to_string())
        .collect()
}

fn get_file_extention(filepath: &str) -> String {
    filepath.split(".").last().unwrap().to_string()
}


fn format_title(file_extention: &str, number: &str, config: &Config) -> String {
    format!("{}E{}.{}",config.path, number, file_extention)
}


fn rename_file(old_path: &str, new_path: &str) {
    if let Err(e) = fs::rename(old_path, new_path) {
        eprintln!("Error renaming file: {}", e);
    }
}
