use std::error::Error;
use std::{env, fs};
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    // new tends to represent objects that certainlly can be new, while use build means the process may fail
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? 提前将错误返回给主调函数
    let contents = fs::read_to_string(config.file_path)?;
    // println!("With the text:\n{contents}");
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&'a str> = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}
pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&'a str> = Vec::new();
    let lowercase_q = query.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(&lowercase_q) {
            res.push(line);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use core::prelude::v1::test;

    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
