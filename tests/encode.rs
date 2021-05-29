use smooshedmorse::encode::encode;

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
