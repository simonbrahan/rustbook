use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Two arguments required: search term, file path");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut output = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            output.push(line);
        }
    }

    output
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
    fn build_config_fails_with_fewer_args() {
        let args: Vec<String> = vec![String::from("unused exec path"), String::from("test query")];

        let err = Config::new(&args);

        assert_eq!(
            "Two arguments required: search term, file path",
            err.expect_err("Should error because too few args")
        );
    }

    #[test]
    fn search_returns_one_result() {
        let query = "duct";
        let content = "
Rust:
Safe, fast, productive.
Pick three.";

        assert_eq!(vec!["Safe, fast, productive."], search(query, content));
    }

    #[test]
    fn search_returns_many_results() {
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
    fn search_returns_no_results() {
        let query = "duct";
        let content = "Content does not include the search query.";

        let expect: Vec<&str> = vec![];
        assert_eq!(expect, search(query, content));
    }
}
