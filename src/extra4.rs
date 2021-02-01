use crate::decode::all_words_to_smooshedmerse;
use crate::extra3::filter_words_by_lenght;
use crate::merses::merse_to_morse;
use crate::wordlist::get_all_words;

const LETTERS_NUMBER: usize = 6;

/// protectorate is 12 letters long and encodes to .--..-.----.-.-.----.-..--., which is
/// a palindrome (i.e. the string is the same when reversed). Find the only 13-letter
/// word that encodes to a palindrome.
pub fn run() -> Result<Vec<String>, &'static str> {
    let all_words: Vec<String> = get_all_words();

    let filtered_words = filter_words_by_lenght(all_words, LETTERS_NUMBER);

    log::info!("Converting all words to smooshedmorse...");
    let merse_words = all_words_to_smooshedmerse(&filtered_words);
    log::info!("Converting all words to smooshedmorse: done");

    let palindrome_merse: Vec<usize> = find_palindrome(&merse_words);
    let mut palindrome_words: Vec<String> = Vec::new();
    for i in palindrome_merse.iter().copied() {
        palindrome_words.push(filtered_words[i].clone())
    }

    for i in palindrome_merse {
        log::info!("Found palindrome: {}", merse_to_morse(&merse_words[i]));
    }

    Ok(palindrome_words)
}

fn find_palindrome(smooshedmerses: &Vec<Vec<bool>>) -> Vec<usize> {
    let mut indexes: Vec<usize> = Vec::new();
    for (i, w) in smooshedmerses.iter().enumerate() {
        if is_palindrome(w) {
            indexes.push(i)
        }
    }
    indexes
}

fn is_palindrome(smooshedmerse: &Vec<bool>) -> bool {
    // Rely on dots being false and dashes being true
    let reversed: Vec<bool> = smooshedmerse.iter().rev().copied().collect();
    if *smooshedmerse == reversed {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_palindrome() {
        assert_eq!(
            find_palindrome(&vec![
                vec![true, false],
                vec![true, false, false],
                vec![true, false, true],
                vec![false, true, true, true],
                vec![true, false, false, true],
            ]),
            vec![2, 4]
        );
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(&vec![true, false, true, true, false, true]));
        assert!(is_palindrome(&vec![true, true]));
        assert!(is_palindrome(&vec![false, true, false]));
        assert!(is_palindrome(&vec![false]));
        assert!(is_palindrome(&vec![]));
        assert!(!is_palindrome(&vec![true, true, true, false, false]));
        assert!(!is_palindrome(&vec![false, true]));
    }
}
