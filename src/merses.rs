/// merse: memory efficient morse
/// . = false
/// - = true
use crate::morses;
use morses::{DASH, DOT};
use std::collections::HashMap;
use thiserror::Error;
use tracing::trace;

const FALSE_CHAR: char = DOT;
const TRUE_CHAR: char = DASH;

#[derive(Error, Debug)]
pub enum MorseError {
    #[error("Invalid morse char: `{0}`")]
    InvalidMorseChar(char),
}

fn morse_char_to_merse(morse_char: &str) -> Result<Vec<bool>, MorseError> {
    let mut merse_chars: Vec<bool> = Vec::new();
    for ch in morse_char.chars() {
        merse_chars.push(match ch {
            FALSE_CHAR => false,
            TRUE_CHAR => true,
            _ => {
                return Err(MorseError::InvalidMorseChar(ch));
            }
        });
    }
    trace!("{}->{:?}", morse_char, merse_chars);
    Ok(merse_chars)
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

pub fn smooshedmorse_to_merse(word: &str) -> Result<Vec<bool>, MorseError> {
    let merse = morse_char_to_merse(word)?;
    trace!("{}->{:?}", word, merse);
    Ok(merse)
}

pub fn merse_to_char(merse_ch: Vec<bool>) -> Result<Option<char>, MorseError> {
    for (k, v) in &get_merse_code()? {
        if *v == merse_ch {
            trace!("{:?}->{}", merse_ch, k);
            return Ok(Some(*k));
        }
    }
    Ok(None)
}

pub fn char_to_merse(ch: char) -> Result<Vec<bool>, MorseError> {
    let morse_ch = morses::char_to_morse(ch);
    let merse_ch = morse_char_to_merse(&morse_ch)?;
    trace!("{}->{}", ch, morse_ch);
    Ok(merse_ch)
}

pub fn get_merse_code() -> Result<HashMap<char, Vec<bool>>, MorseError> {
    let morse = morses::get_morse_code();
    let mut merse: HashMap<char, Vec<bool>> = HashMap::new();
    for (k, v) in &morse {
        merse.insert(*k, morse_char_to_merse(v)?);
    }
    trace!("Merse code: {:?}", merse);
    Ok(merse)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_merse_code() {
        assert!(get_merse_code().unwrap().len() == 26);
    }

    #[test]
    fn test_morse_char_to_merse() {
        assert_eq!(morse_char_to_merse(".-").unwrap(), vec![false, true]);
        assert_eq!(
            morse_char_to_merse("..-.").unwrap(),
            vec![false, false, true, false]
        );
        assert_eq!(
            morse_char_to_merse(".---..-.-.-.--").unwrap(),
            vec![
                false, true, true, true, false, false, true, false, true, false, true, false, true,
                true
            ]
        );
    }

    #[test]
    fn test_smooshedmorse_to_merse() {
        assert_eq!(smooshedmorse_to_merse(".-").unwrap(), vec![false, true]);
        assert_eq!(
            smooshedmorse_to_merse("..-.").unwrap(),
            vec![false, false, true, false]
        );
        assert_eq!(
            smooshedmorse_to_merse(".---..-.-.-.--").unwrap(),
            vec![
                false, true, true, true, false, false, true, false, true, false, true, false, true,
                true
            ]
        );
    }

    #[test]
    fn test_invalid_smooshedmorse_to_merse() {
        assert!(smooshedmorse_to_merse(".- -").is_err());
    }

    #[test]
    fn test_merse_to_char() {
        assert_eq!(
            merse_to_char(vec![true, false, false, false]).unwrap(),
            Some('b')
        );
        assert_eq!(
            merse_to_char(vec![true, false, false, false, false]).unwrap(),
            None
        );
    }

    #[test]
    fn test_char_to_merse() {
        assert_eq!(char_to_merse('b').unwrap(), vec![true, false, false, false]);
        assert_eq!(char_to_merse('i').unwrap(), vec![false, false]);
    }

    #[test]
    fn test_merse_to_morse() {
        assert_eq!(merse_to_morse(&vec![true, false, true]), "-.-".to_string());
        assert_eq!(merse_to_morse(&vec![]), String::new());
    }
}
