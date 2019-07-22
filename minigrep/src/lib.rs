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

    Ok(())
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
}
