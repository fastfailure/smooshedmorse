use smooshedmorse::encode::encode;

#[test]
fn test_encode() {
    assert_eq!(
        encode("carlotta").unwrap(),
        vec!["-.-..-.-..-..-----.-".to_string()]
    );
    assert_eq!(
        encode("Carlotta").unwrap(),
        vec!["-.-..-.-..-..-----.-".to_string()]
    );
    assert!(
        encode("C3rr3t0").is_err(),
        "Word to be encoded must contain only ASCII alphabetic characters"
    );
    assert!(
        encode("C rr t ").is_err(),
        "Word to be encoded must contain only ASCII alphabetic characters"
    );
}
