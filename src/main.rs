use std::env;
use std::process;

use lightgrep::Config;

fn main() {
    // Validate arguments and handle parsing errors
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    // Run the main logic and handle errors
    if let Err(e) = lightgrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}