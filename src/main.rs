use std::env;
use std::process;

use lightgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    if let Err(e) = lightgrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}