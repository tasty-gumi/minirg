use minirg::Config;
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing the args :{err}");
        process::exit(1);
    });
    println!("Search for {}", config.query);
    println!("In file {}", config.file_path);
    if let Err(e) = minirg::run(config) {
        println!("Application failed with error: {e}");
        process::exit(1);
    }
}
