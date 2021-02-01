use crate::encode::word_to_smooshedmerse;
use crate::merses::smooshedmorse_to_merse;
use crate::morses::validate_morse_str;
use crate::wordlist::get_all_words;

pub fn decode(smooshedmorse_word: String) -> Result<Vec<String>, &'static str> {
    log::info!("Decoding: {}", smooshedmorse_word);
    if !validate_morse_str(&smooshedmorse_word) {
        return Err("Morse string must contain only . and -");
    };
    let merse_word = smooshedmorse_to_merse(&smooshedmorse_word);
    decode_merse(merse_word)
}

pub fn decode_merse(smooshedmerse_word: Vec<bool>) -> Result<Vec<String>, &'static str> {
    let all_words: Vec<String> = get_all_words();

    log::info!("Converting all words to smooshedmorse...");
    let all_merse_words = all_words_to_smooshedmerse(&all_words);
    log::info!("Converting all words to smooshedmorse: done");

    log::info!("Searching for corresponding words...");
    let corresponding_positions: Vec<usize> =
        find_merse_corresponding_words(&smooshedmerse_word, &all_merse_words);
    log::info!("Found: {:?}", corresponding_positions);
    let mut res: Vec<String> = Vec::new();
    for i in corresponding_positions {
        res.push(all_words[i].to_string());
    }
    Ok(res)
}

pub fn all_words_to_smooshedmerse(all_words: &Vec<String>) -> Vec<Vec<bool>> {
    // this is the slow function
    let mut all_merse_words: Vec<Vec<bool>> = Vec::new();
    for word in all_words {
        let merse_word: Vec<bool> = word_to_smooshedmerse(word)
            .expect("Word with forbidden characters present in the word list");
        log::debug!("Converted: {}", word);
        all_merse_words.push(merse_word);
    }
    all_merse_words
}

pub fn find_merse_corresponding_words(
    merse_word: &Vec<bool>,
    all_merse_words: &Vec<Vec<bool>>,
) -> Vec<usize> {
    let mut indexes: Vec<usize> = Vec::new();
    for (i, word) in all_merse_words.into_iter().enumerate() {
        if word == merse_word {
            log::debug!("Found: {:?} at {}", word, i);
            indexes.push(i);
        }
    }
    indexes
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_decode() {}

    #[test]
    fn test_find_merse_corresponding_words() {
        assert_eq!(
            find_merse_corresponding_words(
                &vec![true, false],
                &vec![
                    vec![true, false],
                    vec![false, true],
                    vec![true, false],
                    vec![true, false, true],
                    vec![false],
                ]
            ),
            vec![0, 2]
        );
    }

    #[test]
    fn test_all_words_to_smoshedmerse() {
        assert_eq!(
            all_words_to_smooshedmerse(&vec![
                "Lancelot".to_string(),
                "Ginevra".to_string(),
                "a".to_string(),
                "horse".to_string()
            ]),
            vec![
                vec![
                    false, true, false, false, false, true, true, false, true, false, true, false,
                    false, false, true, false, false, true, true, true, true
                ],
                vec![
                    true, true, false, false, false, true, false, false, false, false, false, true,
                    false, true, false, false, true
                ],
                vec![false, true],
                vec![
                    false, false, false, false, true, true, true, false, true, false, false, false,
                    false, false
                ]
            ]
        );
    }

    // #[test]
    // fn test_decode_merse() {
    // too long...
    // assert_eq!(decode_merse(), Ok());
    // }
}
