use smooshedmorse::decode::decode;
use smooshedmorse::decode::decode_merse;

#[test]
fn test_decode() {
    assert_eq!(
        decode("-.-..-.-..-", Some("tests/wordlist")).unwrap(),
        vec![String::from("caret"), String::from("ceca")]
    );
}

#[test]
fn test_decode_merse() {
    assert_eq!(
        decode_merse(
            vec![true, false, true, false, false, true, false, true, false, false, true],
            Some("tests/wordlist")
        )
        .unwrap(),
        vec![String::from("caret"), String::from("ceca")]
    );
}
