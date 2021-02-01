use crate::decode::all_words_to_smooshedmerse;
use crate::decode::find_merse_corresponding_words;
use crate::merses::merse_to_morse;
use crate::wordlist::get_all_words;

const DASHES_NUMBER_SEARCHED: u32 = 15;

/// autotomous encodes to .-..--------------..-..., which has 14 dashes in a row. Find
/// the only word that has 15 dashes in a row.
pub fn run() -> Result<Vec<String>, &'static str> {
    let all_words: Vec<String> = get_all_words();

    log::info!("Converting all words to smooshedmorse...");
    let all_merse_words = all_words_to_smooshedmerse(&all_words);
    log::info!("Converting all words to smooshedmorse: done");

    let many_dashes_word: &Vec<bool> = find_first_with_many_dashes(&all_merse_words)
        .expect("Couldn't find a word with so many dashes!");
    let morse_many_dashes_word: String = merse_to_morse(many_dashes_word);

    log::info!(
        "Found a word with {} dashes: {}",
        DASHES_NUMBER_SEARCHED,
        morse_many_dashes_word
    );

    // display corresponding words in log
    let corresponding_positions: Vec<usize> =
        find_merse_corresponding_words(many_dashes_word, &all_merse_words);
    log::info!("Found: {:?}", corresponding_positions);
    let mut decoded: Vec<String> = Vec::new();
    for i in corresponding_positions {
        decoded.push(all_words[i].to_string());
    }
    log::info!("{}: {:?}", morse_many_dashes_word, decoded);
    Ok(vec![morse_many_dashes_word])
}

fn find_first_with_many_dashes(all_merse_words: &Vec<Vec<bool>>) -> Option<&Vec<bool>> {
    for word in all_merse_words {
        if count_consecutive_dashes(word) >= DASHES_NUMBER_SEARCHED {
            return Some(word);
        }
    }
    return None;
}

fn count_consecutive_dashes(merse_word: &Vec<bool>) -> u32 {
    let mut count: u32 = 0;
    let mut higher_count: u32 = 0;
    for i in merse_word {
        if *i == true {
            count += 1
        } else {
            count = 0
        }
        if count >= higher_count {
            higher_count = count
        }
    }
    higher_count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_consecutive_dashes() {
        assert_eq!(count_consecutive_dashes(&vec![]), 0);
        assert_eq!(count_consecutive_dashes(&vec![false, false, false]), 0);
        assert_eq!(count_consecutive_dashes(&vec![false, true, false]), 1);
        assert_eq!(
            count_consecutive_dashes(&vec![true, false, true, true, true, false]),
            3
        );
    }
    #[test]
    fn test_find_first_with_many_dashes() {
        assert_eq!(find_first_with_many_dashes(&vec![]), None);
        assert_eq!(
            find_first_with_many_dashes(&vec![vec![false, false], vec![true, false, true]]),
            None
        );
        let mdw: Vec<bool> = vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true,
        ];
        assert_eq!(
            find_first_with_many_dashes(&vec![
                vec![false, false],
                vec![true, false, true],
                vec![
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true,
                ]
            ]),
            Some(&mdw)
        );
    }
}
