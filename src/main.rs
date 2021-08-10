// use crate::input::InputLines;
use clap::{App, AppSettings, SubCommand};
use color_eyre::Report;
use serde_json::json;
use tracing::trace;
use tracing_subscriber::EnvFilter;

use smooshedmorse::decode;
use smooshedmorse::encode;
use smooshedmorse::extra1;
use smooshedmorse::extra2;
use smooshedmorse::extra3;
use smooshedmorse::extra4;
use smooshedmorse::permutations;

fn main() -> Result<(), Report> {
    setup()?;
    let matches = App::new("smooshedmorse")
        .about("Smooshed Morse encoding and decoding
Normally, you would indicate where one letter ends and the next begins, for instance
with a space between the letters' codes, but in smooshed morse  all the
coded letters are smooshed together into a single string consisting of only dashes and dots.
https://www.reddit.com/r/dailyprogrammer/comments/cmd1hb/20190805_challenge_380_easy_smooshed_morse_code_1/")
        .setting(AppSettings::SubcommandRequiredElseHelp)
                .arg_from_usage("-j, --json 'Serialize output to JSON'")
        .subcommand(
            SubCommand::with_name("encode")
                .about("Encode a word to smooshedmorse.\nExample:\nsmooshedmorse encode Horse")
                .arg_from_usage("<WORD> 'Word to be encoded to smooshedmorse'")
        )
        .subcommand(
            SubCommand::with_name("decode")
                .about("Decode a smooshedmorse English word.\nExample:\nsmooshedmorse decode -- '....---.-.....'")
                .arg_from_usage("-w, --words=[FILE] 'Word list file to use'")
                .arg_from_usage("<SMOOSHEDMORSE> 'Smooshedmorse word to decode (give it after --)'")
        )
        .subcommand(
            SubCommand::with_name("permutations")
                .about("Given a smooshed Morse code encoding of a permutation of the alphabet, find one of the permutations it encodes. Implement smooshedmorse challenge 2: https://www.reddit.com/r/dailyprogrammer/comments/cn6gz5/20190807_challenge_380_intermediate_smooshed/\nExample:\nsmooshedmorse permutations -- '.--...-.-.-.....-.--........----.-.-..---.---.--.--.-.-....-..-...-.---..--.----..'")
                .arg_from_usage(
                    "[ALPHABET_PERMUTATION] 'Smooshedmorse alphabet permutation to decode, if not given a random one is generated'"
                    )
        )
        .subcommand(
            SubCommand::with_name("extra1")
        )
        .subcommand(
            SubCommand::with_name("extra2")
        )
        .subcommand(
            SubCommand::with_name("extra3")
        )
        .subcommand(
            SubCommand::with_name("extra4")
        )
        .get_matches();
    trace!(?matches);
    match matches.subcommand() {
        ("encode", Some(submatches)) => {
            trace!(?submatches);
            // day5::star1(InputLines::from(submatches.value_of(input_par)))?;
            let res = encode::encode(submatches.value_of("WORD").unwrap())?; // safe unwrap, positional argument is mandatory
            if matches.is_present("json") {
                print_json(&res)
            } else {
                print_result(&res);
            }
        }
        ("decode", Some(submatches)) => {
            trace!(?submatches);
            let res = decode::decode(
                submatches.value_of("SMOOSHEDMORSE").unwrap(),
                submatches.value_of("words"),
            )?; // idem
            if matches.is_present("json") {
                print_json(&res)
            } else {
                print_result(&res);
            }
        }
        ("permutations", Some(submatches)) => {
            trace!(?submatches);
            let res = permutations::run(submatches.value_of("ALPHABET_PERMUTATION"))?;
            if matches.is_present("json") {
                print_json(&res)
            } else {
                print_result(&res);
            }
        }
        ("extra1", Some(submatches)) => {
            trace!(?submatches);
            let res = extra1::run()?;
            if matches.is_present("json") {
                print_json(&res)
            } else {
                print_result(&res);
            }
        }
        ("extra2", Some(submatches)) => {
            trace!(?submatches);
            let res = extra2::run()?;
            if matches.is_present("json") {
                print_json(&res)
            } else {
                print_result(&res);
            }
        }
        ("extra3", Some(submatches)) => {
            trace!(?submatches);
            let res = extra3::run()?;
            if matches.is_present("json") {
                print_json(&res)
            } else {
                print_result(&res);
            }
        }
        ("extra4", Some(submatches)) => {
            trace!(?submatches);
            let res = extra4::run()?;
            if matches.is_present("json") {
                print_json(&res)
            } else {
                print_result(&res);
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn,smooshedmorse=warn")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}

fn print_json(words: &[String]) {
    let json_words = json!(words);
    println!("{}", json_words);
}

fn print_result(words: &[String]) {
    for word in words {
        println!("{}", word);
    }
}
