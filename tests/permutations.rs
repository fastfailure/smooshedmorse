use smooshedmorse::encode::encode;
use smooshedmorse::permutations::run;

#[test]
fn test_permutations() {
    let example_smalpha =
        ".--...-.-.-.....-.--........----.-.-..---.---.--.--.-.-....-..-...-.---..--.----.."; // => "wirnbfzehatqlojpgcvusyxkmd")
    let a_res = run(Some(example_smalpha)).unwrap()[0].clone();
    let encoded = encode(&a_res).unwrap()[0].clone();
    assert_eq!(encoded, example_smalpha);
}
