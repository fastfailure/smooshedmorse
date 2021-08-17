use crate::merses::char_to_merse;
use crate::merses::merse_to_morse;
use crate::morses::char_to_morse;
use color_eyre::{eyre::eyre, Report};
use tracing::{debug, trace};

pub fn encode(word: &str) -> Result<Vec<String>, Report> {
    debug!("Encoding: {}", word);
    let smooshedmerse = word_to_smooshedmerse(word)?;
    Ok(vec![merse_to_morse(&smooshedmerse)])
}

pub fn word_to_smooshedmerse(word: &str) -> Result<Vec<bool>, Report> {
    validate_ascii(&word)?;
    let mut merse: Vec<Vec<bool>> = Vec::new();
    for ch in word.chars() {
        let mc = char_to_merse(ch)?;
        merse.push(mc);
    }
    let encoded: Vec<bool> = merse.into_iter().flatten().collect();
    trace!("{}->{:?}", word, encoded);
    Ok(encoded)
}

pub fn word_to_smooshedmorse(word: &str) -> Result<String, Report> {
    validate_ascii(&word)?;
    let mut morse_vec: Vec<String> = Vec::new();
    for ch in word.chars() {
        let mc = char_to_morse(ch);
        morse_vec.push(mc);
    }
    let encoded: String = morse_vec.into_iter().collect();
    Ok(encoded)
}

pub fn validate_ascii(word: &str) -> Result<(), Report> {
    if !word.chars().all(char::is_alphabetic) {
        return Err(eyre!(
            "Word doesn't contain only ASCII alphabetic characters: `{}`",
            word
        ));
    }
    trace!("{} is valid", word);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate() {
        assert!(validate_ascii("Carlotta").is_ok());
        assert!(validate_ascii("supercalifragilistichespiralidoso").is_ok());
        assert!(validate_ascii("S&C").is_err());
        assert!(validate_ascii("S4ndr0").is_err());
        assert!(validate_ascii("AB♡").is_err());
        assert!(validate_ascii("Sandro ").is_err());
        assert!(validate_ascii("S C").is_err());
    }

    #[test]
    fn test_word_to_smooshedmorse() {
        assert!(word_to_smooshedmorse("a ").is_err());
        assert!(word_to_smooshedmorse("b b").is_err());
        assert_eq!(word_to_smooshedmorse("").unwrap(), String::from(""));
        assert_eq!(
            word_to_smooshedmorse("Carlotta").unwrap(),
            String::from("-.-..-.-..-..-----.-")
        )
    }

    #[test]
    fn test_word_to_smooshedmerse() {
        assert!(word_to_smooshedmerse("a ").is_err());
        assert!(word_to_smooshedmerse("b b").is_err());
        assert_eq!(word_to_smooshedmerse("").unwrap(), Vec::<bool>::new());
        assert_eq!(
            word_to_smooshedmerse("Carlotta").unwrap(),
            vec![
                true, false, true, false, false, true, false, true, false, false, true, false,
                false, true, true, true, true, true, false, true
            ]
        );
    }
}
