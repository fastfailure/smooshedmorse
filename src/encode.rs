use crate::merses::char_to_merse;
use crate::merses::merse_to_morse;
use crate::morses::char_to_morse;

pub fn encode(word: String) -> Result<Vec<String>, &'static str> {
    log::debug!("Encoding: {}", word);
    let smooshedmerse = word_to_smooshedmerse(&word);
    match smooshedmerse {
        Err(err) => Err(err),
        Ok(sm) => Ok(vec![merse_to_morse(&sm)]),
    }
}

pub fn word_to_smooshedmerse(word: &str) -> Result<Vec<bool>, &'static str> {
    if !validate_ascii(&word) {
        return Err("Word to be encoded must contain only ASCII alphabetic characters");
    };

    let mut merse: Vec<Vec<bool>> = Vec::new();
    for ch in word.chars() {
        let mc = char_to_merse(ch);
        merse.push(mc);
    }
    let encoded: Vec<bool> = merse.into_iter().flatten().collect();
    log::trace!("{}->{:?}", word, encoded);
    Ok(encoded)
}

pub fn word_to_smooshedmorse(word: &str) -> Result<String, &'static str> {
    if !validate_ascii(&word) {
        return Err("Word to be encoded must contain only ASCII alphabetic characters");
    };

    let mut morse_vec: Vec<String> = Vec::new();
    for ch in word.chars() {
        let mc = char_to_morse(ch);
        morse_vec.push(mc);
    }
    let encoded: String = morse_vec.into_iter().collect();
    Ok(encoded)
}

pub fn validate_ascii(word: &str) -> bool {
    if !word.chars().all(char::is_alphabetic) {
        log::error!(
            "Word `{}` is NOT valid, it must contain only ASCII alphabetic characters",
            word
        );
        return false;
    }
    log::trace!("{} is valid", word);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate() {
        assert!(validate_ascii("Carlotta"));
        assert!(validate_ascii("supercalifragilistichespiralidoso"));
        assert!(!validate_ascii("S&C"));
        assert!(!validate_ascii("S4ndr0"));
        assert!(!validate_ascii("AB♡"));
        assert!(!validate_ascii("Sandro "));
        assert!(!validate_ascii("S C"));
    }

    #[test]
    fn test_word_to_smooshedmorse() {
        assert!(word_to_smooshedmorse("a ").is_err());
        assert!(word_to_smooshedmorse("b b").is_err());
        assert_eq!(word_to_smooshedmorse(""), Ok(String::from("")));
        assert_eq!(
            word_to_smooshedmorse("Carlotta"),
            Ok(String::from("-.-..-.-..-..-----.-"))
        )
    }

    #[test]
    fn test_word_to_smooshedmerse() {
        assert!(word_to_smooshedmerse("a ").is_err());
        assert!(word_to_smooshedmerse("b b").is_err());
        assert_eq!(word_to_smooshedmerse(""), Ok(vec![]));
        assert_eq!(
            word_to_smooshedmerse("Carlotta"),
            Ok(vec![
                true, false, true, false, false, true, false, true, false, false, true, false,
                false, true, true, true, true, true, false, true
            ])
        );
    }
}
