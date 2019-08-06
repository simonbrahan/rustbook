use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("At least two arguments required: search term, file path");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let insensitive_flag_passed = args.len() > 3 && args[3] == "i";
        let case_sensitive =
            env::var("MINIGREP_CASE_INSENSITIVE").is_err() && !insensitive_flag_passed;

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let output = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in output {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_config_from_args() {
        let args: Vec<String> = vec![
            String::from("unused exec path"),
            String::from("test query"),
            String::from("path/to/file.txt"),
        ];

        let conf = Config::new(&args).expect("Should be a Config struct");

        assert_eq!("test query", conf.query);
        assert_eq!("path/to/file.txt", conf.filename);
    }

    #[test]
    fn build_config_with_passed_case_insensitive() {
        let args: Vec<String> = vec![
            String::from("unused exec path"),
            String::from("test query"),
            String::from("path/to/file.txt"),
            String::from("i"),
        ];

        let conf = Config::new(&args).expect("Should be a Config struct");

        assert_eq!("test query", conf.query);
        assert_eq!("path/to/file.txt", conf.filename);
        assert!(!conf.case_sensitive);
    }

    #[test]
    fn build_config_fails_with_fewer_args() {
        let args: Vec<String> = vec![String::from("unused exec path"), String::from("test query")];

        let err = Config::new(&args);

        assert_eq!(
            "At least two arguments required: search term, file path",
            err.expect_err("Should error because too few args")
        );
    }

    #[test]
    fn case_sensitive_search_returns_one_result() {
        let query = "duct";
        let content = "
Rust:
Safe, fast, productive.
Pick three.";

        assert_eq!(vec!["Safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_sensitive_search_returns_many_results() {
        let query = "duct";
        let content = "
Rust:
Safe, fast, productive.
Pick three.
Includes duct tape.";

        assert_eq!(
            vec!["Safe, fast, productive.", "Includes duct tape."],
            search(query, content)
        );
    }

    #[test]
    fn case_sensitive_search_returns_no_results() {
        let query = "duct";
        let content = "Content does not include the search query.";

        let expect: Vec<&str> = vec![];
        assert_eq!(expect, search(query, content));
    }

    #[test]
    fn case_insensitive_search_returns_one_result() {
        let query = "duct";
        let content = "
Duct tape.
It's my favourite tape.";

        assert_eq!(vec!["Duct tape."], search_case_insensitive(query, content));
    }
}
