/// Normally, you would indicate where one letter ends and the next begins, for instance
/// with a space between the letters' codes, but for this challenge, just smoosh all the
/// coded letters together into a single string consisting of only dashes and dots.
use std::env;
use std::process;

use smooshedmorse::input::get_config;
use smooshedmorse::run;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let config = get_config(&args).unwrap_or_else(|err| {
        log::error!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    run(config);
}
