use std::{env, process};

use component_generator::config::Config;

fn main() {
    let cli_config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing the arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = component_generator::run(cli_config) {
        eprintln!("Generation error: {}", e);

        process::exit(1);
    }
}
