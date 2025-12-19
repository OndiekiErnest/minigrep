use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
        args.next();

        let Some(query) = args.next() else {
            return Err(String::from("Didn't specify the query string"));
        };
        let Some(file_path) = args.next() else {
            return Err(String::from("Didn't specify the file path"));
        };

        // read environment variable ($Env:IGNORE_CASE=1; cargo run -- to poem.txt)
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn search<'a>(query: &'a str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    contents.lines().filter(move |line| line.contains(query))
}

fn isearch<'a>(query: &'a str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    // convert query to lowercase
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(move |line| line.to_lowercase().contains(&query))
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    if config.ignore_case {
        for line in isearch(&config.query, &contents) {
            println!("{line}");
        }
    } else {
        for line in search(&config.query, &contents) {
            println!("{line}");
        }
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let results: Vec<&str> = search(query, contents).collect();
        assert_eq!(vec!["safe, fast, productive."], results);
    }

    #[test]
    fn search_case_insensitive() {
        let query = "DUCT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let results: Vec<&str> = isearch(query, contents).collect();
        assert_eq!(vec!["safe, fast, productive."], results);
    }
}
