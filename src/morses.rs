use color_eyre::{eyre::eyre, Report};
use std::collections::HashMap;
use tracing::{error, trace};

pub const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
pub const DOT: char = '.';
pub const DASH: char = '-';
pub const MORSE: &str = ".- -... -.-. -.. . ..-. --. .... .. .--- -.- .-.. -- -. --- .--. --.- .-. ... - ..- ...- .-- -..- -.-- --..";

pub fn get_morse_code() -> HashMap<char, &'static str> {
    let morse_seq: Vec<&str> = MORSE.split(' ').collect();
    let mc: HashMap<char, &str> = ALPHABET
        .iter()
        .copied()
        .zip(morse_seq.into_iter())
        .collect();
    trace!("Morse: {:?}", mc);
    mc
}

pub fn morse_to_char(morse_ch: &str) -> Option<char> {
    for (k, v) in &get_morse_code() {
        if *v == morse_ch {
            trace!("{}->{}", morse_ch, k);
            return Some(*k);
        }
    }
    None
}

pub fn char_to_morse(ch: char) -> String {
    let morse_ch = get_morse_code()
        .get(&ch.to_ascii_lowercase())
        .expect("The character given is not present in morse code")
        .to_string();
    trace!("{}->{}", ch, morse_ch);
    morse_ch
}

/// Valid morse string must contain only . and -
pub fn validate_morse_str(morse_str: &str) -> Result<(), Report> {
    if morse_str.chars().all(|c| matches!(c, DOT | DASH)) {
        trace!("{} is a valid morse string", morse_str);
        return Ok(());
    }
    error!(%morse_str, "Morse string must contain only . and -");
    Err(eyre!("Not a valid morse string: `{}`", morse_str))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate() {
        assert!(validate_morse_str("-").is_ok());
        assert!(validate_morse_str(".").is_ok());
        assert!(validate_morse_str("-.").is_ok());
        assert!(validate_morse_str(".-.........................--------------------").is_ok());
        assert!(validate_morse_str(".--.-").is_ok());
        assert!(validate_morse_str(".--.- ").is_err());
        assert!(validate_morse_str(".- .-.").is_err());
        assert!(validate_morse_str("a").is_err());
        assert!(validate_morse_str("9").is_err());
        assert!(validate_morse_str("!").is_err());
        assert!(validate_morse_str("-.3-").is_err());
        assert!(validate_morse_str("-♡").is_err());
        assert!(validate_morse_str("_").is_err());
        assert!(validate_morse_str("_.").is_err());
    }

    #[test]
    fn test_get_morse_code() {
        assert!(get_morse_code().len() == 26);
    }

    #[test]
    fn test_char_to_morse() {
        assert_eq!(char_to_morse('a'), ".-".to_string());
        assert_eq!(char_to_morse('k'), "-.-".to_string());
        assert_eq!(char_to_morse('z'), "--..".to_string());
        assert_eq!(char_to_morse('S'), "...".to_string());
    }

    #[test]
    #[should_panic]
    fn test_invalid_char_to_morse() {
        char_to_morse('à');
    }

    #[test]
    fn test_morse_to_char() {
        assert_eq!(morse_to_char("-."), Some('n'));
        assert_eq!(morse_to_char("..."), Some('s'));
        assert_eq!(morse_to_char("---"), Some('o'));
        assert_eq!(morse_to_char("----"), None);
    }
}
