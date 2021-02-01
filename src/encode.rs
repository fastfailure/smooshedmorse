use crate::merses::char_to_merse;
use crate::merses::merse_to_morse;
use crate::morses::char_to_morse;

pub fn encode(word: String) -> Result<Vec<String>, &'static str> {
    log::info!("Encoding: {}", word);
    let smooshedmerse = word_to_smooshedmerse(&word);
    match smooshedmerse {
        Err(err) => return Err(err),
        Ok(sm) => return Ok(vec![merse_to_morse(&sm)]),
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
    let encoded: Vec<bool> = merse.into_iter().flatten().collect(); // <3
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
    if !word
        .chars()
        // .filter(|c| !c.is_ascii_alphabetic())
        // .collect::<Vec<_>>();
        .all(char::is_alphabetic)
    {
        // log::info!("Non ASCII alphabetic chars: {:?}", non_ascii_alphabetic);
        // if !non_ascii_alphabetic.is_empty() {
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
    fn test_encode() {
        assert_eq!(
            encode("carlotta".to_string()),
            Ok(vec!["-.-..-.-..-..-----.-".to_string()])
        );
        assert_eq!(
            encode("Carlotta".to_string()),
            Ok(vec!["-.-..-.-..-..-----.-".to_string()])
        );
        assert!(
            encode("C3rr3t0".to_string()).is_err(),
            "Word to be encoded must contain only ASCII alphabetic characters"
        );
        assert!(
            encode("C rr t ".to_string()).is_err(),
            "Word to be encoded must contain only ASCII alphabetic characters"
        );
    }

    // #[test]
    // pub fn word_to_smooshedmerse(word: &str) -> Result<Vec<bool>, &'static str> {
}
// pub fn word_to_smooshedmorse(word: &str) -> Result<String, &'static str> {
