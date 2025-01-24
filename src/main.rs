use minirg::Config;
use std::process;
fn main() {
    let config = Config::build(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing the args :{err}");
        process::exit(1);
    });
    println!("Search for {}", config.query);
    println!("In file {}", config.file_path);
    if let Err(e) = minirg::run(config) {
        eprintln!("Application failed with error: {e}");
        process::exit(1);
    }
}
