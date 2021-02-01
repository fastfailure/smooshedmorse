/// Normally, you would indicate where one letter ends and the next begins, for instance
/// with a space between the letters' codes, but for this challenge, just smoosh all the
/// coded letters together into a single string consisting of only dashes and dots.
mod decode;
mod encode;
mod extra1;
mod extra2;
mod extra3;
mod extra4;
mod extra5;
mod input;
mod merses;
mod morses;
mod wordlist;

use input::get_config;
use input::Config;
use input::Method;
use serde_json::json;
use std::env;
use std::process;

fn print_result(words: Vec<String>) {
    let json_words = json!(words);
    println!("{}", json_words);
}

fn run(config: Config) {
    let res = match config.method {
        Method::Encode => encode::encode(config.word.unwrap().clone()),
        Method::Decode => decode::decode(config.word.unwrap().clone()),
        Method::Extra1 => extra1::run(),
        Method::Extra2 => extra2::run(),
        Method::Extra3 => extra3::run(),
        Method::Extra4 => extra4::run(),
        Method::Extra5 => extra5::run(),
    };
    if let Err(e) = res {
        log::error!("Application error: {}", e);
        process::exit(1);
    };
    print_result(res.unwrap());
}

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let config = get_config(&args).unwrap_or_else(|err| {
        log::error!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    run(config);
}
