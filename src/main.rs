use serde_json::json;
use std::env;
use std::process;

use smooshedmorse::decode;
use smooshedmorse::encode;
use smooshedmorse::extra1;
use smooshedmorse::extra2;
use smooshedmorse::extra3;
use smooshedmorse::extra4;
use smooshedmorse::input::get_config;
use smooshedmorse::input::Config;
use smooshedmorse::input::Method;

pub fn run(config: Config) -> Result<Vec<String>, &'static str> {
    let res = match config.method {
        Method::Encode => encode::encode(config.word.unwrap()),
        Method::Decode => decode::decode(config.word.unwrap()),
        Method::Extra1 => extra1::run(),
        Method::Extra2 => extra2::run(),
        Method::Extra3 => extra3::run(),
        Method::Extra4 => extra4::run(),
    };
    if let Err(e) = res {
        log::error!("Application error: {}", e);
        process::exit(1);
    };
    res
}

fn print_result(words: Vec<String>) {
    let json_words = json!(words);
    println!("{}", json_words);
}

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let config = get_config(&args).unwrap_or_else(|err| {
        log::error!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    print_result(run(config).unwrap());
}
