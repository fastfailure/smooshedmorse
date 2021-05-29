use smooshedmorse::decode::decode;
use smooshedmorse::decode::decode_merse;

// mock use crate::wordlist::get_all_words;

#[test]
fn test_decode() {
    assert_eq!(
        decode(String::from("-.-..-.-..-")),
        Ok(vec![String::from("caret"), String::from("ceca")])
    );
}

#[test]
fn test_decode_merse() {
    assert_eq!(
        decode_merse(vec![
            true, false, true, false, false, true, false, true, false, false, true
        ]),
        Ok(vec![String::from("caret"), String::from("ceca")])
    );
}