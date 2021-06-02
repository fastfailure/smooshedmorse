use smooshedmorse::encode::encode;
use smooshedmorse::permutations::run;

#[test]
fn test_permutations() {
    env_logger::init();
    let example_smalpha =
        ".--...-.-.-.....-.--........----.-.-..---.---.--.--.-.-....-..-...-.---..--.----.."; // => "wirnbfzehatqlojpgcvusyxkmd")
    let a_res = run(Some(example_smalpha.to_string())).unwrap()[0].clone();
    let encoded = encode(a_res).unwrap()[0].clone();
    assert_eq!(encoded, example_smalpha);
}
