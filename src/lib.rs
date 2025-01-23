use std::error::Error;
use std::fs;
pub struct Config {
    pub query: String,
    pub file_path: String,
}
impl Config {
    // new tends to represent objects that certainlly can be new, while use build means the process may fail
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? 提前将错误返回给主调函数
    let contents = fs::read_to_string(config.file_path)?;
    println!("With the text:\n{contents}");
    Ok(())
}
