use crate::decode::all_words_to_smooshedmerse;
use crate::decode::find_merse_corresponding_words;
use crate::merses::merse_to_morse;
use crate::wordlist::get_all_words;
use std::collections::HashMap;

const MAX_FREQUENCY_ALLOWED: Option<u32> = None;
const STARTING_FREQUENCY: u32 = 0; // higher to optimize search, 12 is the highest
                                   // of course these should be a parameters

/// The sequence -...-....-.--. is the code for four different words (needing, nervate,
/// niding, tiling). Find the only sequence that's the code for 13 different words.
pub fn run() -> Result<Vec<String>, &'static str> {
    let all_words: Vec<String> = get_all_words();

    log::info!("Converting all words to smooshedmorse...");
    let all_merse_words = all_words_to_smooshedmerse(&all_words);
    log::info!("Converting all words to smooshedmorse: done");

    log::info!("Counting occurrences...");
    let counted = count_smooshedmorse(&all_merse_words);
    let most_frequent = get_higher_counted(counted);

    // display corresponding words in log
    for code in most_frequent.iter() {
        let corresponding_positions: Vec<usize> =
            find_merse_corresponding_words(&code, &all_merse_words);
        log::info!("Found: {:?}", corresponding_positions);
        let mut decoded: Vec<String> = Vec::new();
        for i in corresponding_positions {
            decoded.push(all_words[i].to_string());
        }
        log::info!("{}: {:?}", merse_to_morse(&code), decoded);
    }

    let morse_most_frequent: Vec<String> =
        most_frequent.iter().map(|m| merse_to_morse(m)).collect();
    log::info!("Most frequently occurring codes: {:?}", morse_most_frequent);
    Ok(morse_most_frequent)
}

/// Return a map of each input code and his frequency
fn count_smooshedmorse(all_merse_words: &[Vec<bool>]) -> HashMap<Vec<bool>, u32> {
    let mut map: HashMap<Vec<bool>, u32> = HashMap::new();
    for c in all_merse_words.iter().cloned() {
        *map.entry(c).or_insert(0) += 1;
    }
    map
}

/// Return all merse codes that have the highest frequency
fn get_higher_counted(merse_count_map: HashMap<Vec<bool>, u32>) -> Vec<Vec<bool>> {
    let mut higher_count: u32 = STARTING_FREQUENCY;
    let mut higher_codes: Vec<Vec<bool>> = Vec::new();
    for (m, c) in merse_count_map {
        if let Some(max) = MAX_FREQUENCY_ALLOWED {
            if c > max {
                continue;
            }
        }
        if c > higher_count {
            log::trace!("Got a new higher count: {}", c);
            higher_count = c;
            higher_codes = Vec::new();
        }
        if c == higher_count {
            log::trace!("Adding to highest frequency ({}): {:?}", c, m);
            higher_codes.push(m);
        }
    }
    higher_codes
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_run()
    //  fn run() -> Result<Vec<String>, &'static str> {

    #[test]
    fn test_count_smooshedmorse() {
        let mut testmap: HashMap<Vec<bool>, u32> = HashMap::new();
        testmap.insert(vec![false, false, false], 1);
        testmap.insert(vec![false, true], 1);
        testmap.insert(vec![true], 2);
        testmap.insert(vec![true, false], 3);
        assert_eq!(
            count_smooshedmorse(&vec![
                vec![true],
                vec![true, false],
                vec![true, false],
                vec![false, true],
                vec![false, false, false],
                vec![true, false],
                vec![true]
            ]),
            testmap
        );
    }

    #[test]
    fn test_get_higher_counted() {
        let mut testmap: HashMap<Vec<bool>, u32> = HashMap::new();
        testmap.insert(vec![false, false, false], 1);
        testmap.insert(vec![false, true], 1);
        testmap.insert(vec![true], 2);
        testmap.insert(vec![true, false], 3);
        assert_eq!(get_higher_counted(testmap), vec![vec![true, false]]);
    }
}
