use std::env;
use std::process;
use serde_json::json;

use smooshedmorse::input::get_config;
use smooshedmorse::run;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let config = get_config(&args).unwrap_or_else(|err| {
        log::error!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    print_result(run(config).unwrap());
}

fn print_result(words: Vec<String>) {
    let json_words = json!(words);
    println!("{}", json_words);
}
