pub mod decode;
pub mod encode;
pub mod input;
pub mod merses;
pub mod morses;

mod extra1;
mod extra2;
mod extra3;
mod extra4;
mod extra5;
mod wordlist;

use input::Config;
use input::Method;
use serde_json::json;
use std::process;

fn print_result(words: Vec<String>) {
    let json_words = json!(words);
    println!("{}", json_words);
}

pub fn run(config: Config) {
    let res = match config.method {
        // Method::Encode => encode::encode(config.word.unwrap().clone()),
        // Method::Decode => decode::decode(config.word.unwrap().clone()),
        Method::Encode => encode::encode(config.word.unwrap()),
        Method::Decode => decode::decode(config.word.unwrap()),
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
