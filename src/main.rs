use crate::input::InputLines;
use clap::{App, AppSettings, SubCommand};
use color_eyre::Report;
use serde_json::json;
use std::env;
use std::process;
use tracing::error;
use tracing::trace;
use tracing_subscriber::EnvFilter;

use smooshedmorse::decode;
use smooshedmorse::encode;
use smooshedmorse::extra1;
use smooshedmorse::extra2;
use smooshedmorse::extra3;
use smooshedmorse::extra4;
use smooshedmorse::input::get_config;
use smooshedmorse::input::Config;
use smooshedmorse::input::Method;
use smooshedmorse::permutations;

pub fn run(config: Config) -> Result<Vec<String>, &'static str> {
    let res = match config.method {
        Method::Encode => encode::encode(config.word.unwrap()),
        Method::Decode => decode::decode(config.word.unwrap()),
        Method::Extra1 => extra1::run(),
        Method::Extra2 => extra2::run(),
        Method::Extra3 => extra3::run(),
        Method::Extra4 => extra4::run(),
        Method::Permutations => permutations::run(config.word),
    };
    if let Err(e) = res {
        error!("Application error: {}", e);
        process::exit(1);
    };
    res
}

fn print_result(words: Vec<String>) {
    let json_words = json!(words);
    println!("{}", json_words);
}

fn main() -> Result<(), Report> {
    setup()?;

    let doc = "
Usage:
smooshedmorse encode <English word>
sdecodemooshedmorse decode <Smooshedmorse word>
smooshedmorse [extra1|extra2|extra3|extra4]
smooshedmorse permutations [<smooshedmorse alphabet permutation>]

permutations command implement smooshedmorse challenge 2: https://www.reddit.com/r/dailyprogrammer/comments/cn6gz5/20190807_challenge_380_intermediate_smooshed/
If no alphabet permutation is given a random one is used.

 Examples:
smooshedmorse encode Horse
smooshedmorse decode ....---.-.....
smooshedmorse permutations .--...-.-.-.....-.--........----.-.-..---.---.--.--.-.-....-..-...-.---..--.----..
";
    let args: Vec<String> = env::args().collect();
    let config = get_config(&args).unwrap_or_else(|err| {
        error!("Problem parsing arguments: {}", err);
        println!("{}", doc);
        process::exit(1);
    });
    print_result(run(config).unwrap());

    let input_par = "INPUT";
    let common_input_arg = "[INPUT] 'Sets the input file to use, read from stdin otherwise'";
    let matches = App::new("aoc18")
        .about("Advent of Code 2018: https://adventofcode.com/2018")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("day1")
                .about("Day 1 challenges")
                .arg_from_usage(common_input_arg),
        )
        .get_matches();
    trace!(?matches);
    match matches.subcommand() {
        ("day5", Some(submatches)) => {
            trace!(?submatches);
            day5::star1(InputLines::from(submatches.value_of(input_par)))?;
            day5::star2(InputLines::from(submatches.value_of(input_par)))?;
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
        std::env::set_var("RUST_LOG", "warn,aoc18=info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
