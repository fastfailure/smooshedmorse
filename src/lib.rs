/// Normally, you would indicate where one letter ends and the next begins, for instance
/// with a space between the letters' codes, but for this challenge, just smoosh all the
/// coded letters together into a single string consisting of only dashes and dots.
pub mod decode;
pub mod encode;
pub mod input;
pub mod merses;
pub mod morses;

mod extra1;
mod extra2;
mod extra3;
mod extra4;
mod wordlist;

use std::process;

use input::Config;
use input::Method;

pub fn run(config: Config) -> Result<Vec<String>, &'static str> {
    let res = match config.method {
        // Method::Encode => encode::encode(config.word.unwrap().clone()),
        // Method::Decode => decode::decode(config.word.unwrap().clone()),
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
