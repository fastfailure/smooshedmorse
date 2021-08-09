/// merse: memory efficient morse
/// . = false
/// - = true
use crate::morses;
use morses::{DASH, DOT};
use std::collections::HashMap;
use tracing::trace;

const FALSE_CHAR: char = DOT;
const TRUE_CHAR: char = DASH;

fn morse_char_to_merse(morse_char: &str) -> Vec<bool> {
    let mut merse_chars: Vec<bool> = Vec::new();
    for ch in morse_char.chars() {
        merse_chars.push(match ch {
            FALSE_CHAR => false,
            TRUE_CHAR => true,
            _ => panic!(),
        });
    }
    trace!("{}->{:?}", morse_char, merse_chars);
    merse_chars
}

pub fn merse_to_morse(merse: &[bool]) -> String {
    let mut morse = String::new();
    for ch in merse {
        morse.push(match ch {
            false => FALSE_CHAR,
            true => TRUE_CHAR,
        });
    }
    trace!("{:?}->{}", merse, morse);
    morse
}

pub fn smooshedmorse_to_merse(word: &str) -> Vec<bool> {
    let merse = morse_char_to_merse(word);
    trace!("{}->{:?}", word, merse);
    merse
}

pub fn merse_to_char(merse_ch: Vec<bool>) -> Option<char> {
    for (k, v) in &get_merse_code() {
        if *v == merse_ch {
            trace!("{:?}->{}", merse_ch, k);
            return Some(*k);
        }
    }
    None
}

pub fn char_to_merse(ch: char) -> Vec<bool> {
    let morse_ch = morses::char_to_morse(ch);
    let merse_ch = morse_char_to_merse(&morse_ch);
    trace!("{}->{}", ch, morse_ch);
    merse_ch
}

pub fn get_merse_code() -> HashMap<char, Vec<bool>> {
    let morse = morses::get_morse_code();
    let mut merse: HashMap<char, Vec<bool>> = HashMap::new();
    for (k, v) in &morse {
        merse.insert(*k, morse_char_to_merse(v));
    }
    trace!("Merse code: {:?}", merse);
    merse
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_merse_code() {
        assert!(get_merse_code().len() == 26);
    }

    #[test]
    fn test_morse_char_to_merse() {
        assert_eq!(morse_char_to_merse(".-"), vec![false, true]);
        assert_eq!(morse_char_to_merse("..-."), vec![false, false, true, false]);
        assert_eq!(
            morse_char_to_merse(".---..-.-.-.--"),
            vec![
                false, true, true, true, false, false, true, false, true, false, true, false, true,
                true
            ]
        );
    }

    #[test]
    fn test_smooshedmorse_to_merse() {
        assert_eq!(smooshedmorse_to_merse(".-"), vec![false, true]);
        assert_eq!(
            smooshedmorse_to_merse("..-."),
            vec![false, false, true, false]
        );
        assert_eq!(
            smooshedmorse_to_merse(".---..-.-.-.--"),
            vec![
                false, true, true, true, false, false, true, false, true, false, true, false, true,
                true
            ]
        );
    }

    #[test]
    #[should_panic]
    fn test_invalid_smooshedmorse_to_merse() {
        smooshedmorse_to_merse(".- -");
    }

    #[test]
    fn test_merse_to_char() {
        assert_eq!(merse_to_char(vec![true, false, false, false]), Some('b'));
        assert_eq!(merse_to_char(vec![true, false, false, false, false]), None);
    }

    #[test]
    fn test_char_to_merse() {
        assert_eq!(char_to_merse('b'), vec![true, false, false, false]);
        assert_eq!(char_to_merse('i'), vec![false, false]);
    }

    #[test]
    fn test_merse_to_morse() {
        assert_eq!(merse_to_morse(&vec![true, false, true]), "-.-".to_string());
        assert_eq!(merse_to_morse(&vec![]), String::new());
    }
}
