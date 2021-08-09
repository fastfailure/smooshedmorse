use crate::decode::all_words_to_smooshedmerse;
use crate::merses::merse_to_morse;
use crate::wordlist::get_all_words;
use color_eyre::Report;
use tracing::info;

const LETTERS_NUMBER: usize = 21;

/// Call a word perfectly balanced if its code has the same number of dots as dashes.
/// counterdemonstrations is one of two 21-letter words that's perfectly balanced. Find
/// the other one.
pub fn run() -> Result<Vec<String>, Report> {
    let all_words: Vec<String> = get_all_words();

    let filtered_words = filter_words_by_lenght(all_words, LETTERS_NUMBER);

    info!("Converting all words to smooshedmorse...");
    let merse_words = all_words_to_smooshedmerse(&filtered_words);
    info!("Converting all words to smooshedmorse: done");

    let balanced_merse: Vec<usize> = find_balanced(&merse_words);
    let mut balanced_words: Vec<String> = Vec::new();
    for i in balanced_merse.iter().copied() {
        balanced_words.push(filtered_words[i].clone())
    }

    for i in balanced_merse {
        info!("Found balanced: {}", merse_to_morse(&merse_words[i]));
    }

    Ok(balanced_words)
}

pub fn filter_words_by_lenght(words: Vec<String>, letters: usize) -> Vec<String> {
    let mut given_lenght_words: Vec<String> = Vec::new();
    for word in words.iter().cloned() {
        let letterscount: usize = word.chars().count();
        if letterscount == letters {
            given_lenght_words.push(word.clone())
        }
    }
    given_lenght_words
}

fn find_balanced(smooshedmerses: &[Vec<bool>]) -> Vec<usize> {
    let mut indexes: Vec<usize> = Vec::new();
    for (i, w) in smooshedmerses.iter().enumerate() {
        if is_balanced(w) {
            indexes.push(i)
        }
    }
    indexes
}

fn is_balanced(smooshedmerse: &[bool]) -> bool {
    // Return true if given smooshed word has same number of true and false
    // Rely on dots being false and dashes being true
    let mut dotcount: u32 = 0;
    let mut dashcount: u32 = 0;
    for elem in smooshedmerse.iter().copied() {
        if elem {
            dashcount += 1;
        } else {
            dotcount += 1;
        }
    }
    if dotcount == dashcount {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_balanced() {
        assert_eq!(
            find_balanced(&vec![
                vec![true],
                vec![true, false, false],
                vec![true, false],
                vec![false],
                vec![true, false, true, false, false, true],
            ]),
            vec![2, 4]
        );
    }

    #[test]
    fn test_is_balanced() {
        assert!(is_balanced(&vec![true, false, true, true, false, false]));
        assert!(is_balanced(&vec![]));
        assert!(!is_balanced(&vec![true, true, true, false, false]));
        assert!(!is_balanced(&vec![true]));
        assert!(!is_balanced(&vec![false]));
    }

    #[test]
    fn test_filter_words_by_lenght() {
        assert_eq!(
            filter_words_by_lenght(
                vec![
                    "a".to_string(),
                    "bcde".to_string(),
                    "tttt".to_string(),
                    "sntoaehu".to_string(),
                ],
                4
            ),
            vec!["bcde".to_string(), "tttt".to_string(),]
        );
    }
}
