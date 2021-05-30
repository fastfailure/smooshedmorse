/// SmooshedMorse challenge 2
/// Given a smooshed Morse code encoding of a permutation of the alphabet, find the permutation it
/// encodes, or any other permutation that produces the same encoding (in general there will be
/// more than one)
/// https://www.reddit.com/r/dailyprogrammer/comments/cn6gz5/20190807_challenge_380_intermediate_smooshed/
use crate::encode::encode;
use crate::merses::smooshedmorse_to_merse;
use crate::morses::validate_morse_str;
use crate::morses::ALPHABET;
// use rand;
use rand::seq::SliceRandom;
use std::collections::HashMap;

const INCREMENT: u8 = 3;

pub fn run(smooshed_alphabet_permutation: Option<String>) -> Result<Vec<String>, &'static str> {
    let smalpha: String = match smooshed_alphabet_permutation {
        Some(smalpha) => smalpha,
        None => {
            let random_alphabet = random_alphabet();
            log::debug!("Alphabet permutation generated: {}", random_alphabet,);
            let smalpha: String = encode(random_alphabet).unwrap()[0].clone();
            log::info!(
                "Alphabet permutation not given, using a random one: {:?}",
                smalpha,
            );
            smalpha
        }
    };
    validate_smalpha(&smalpha)?;
    let permutations_of_char: Vec<Vec<char>> =
        find_permutations(&smooshedmorse_to_merse(&smalpha), INCREMENT);
    Ok(permutations_of_char
        .iter()
        .map(|alphchars| alphchars.iter().collect::<String>())
        .collect::<Vec<String>>())
}

fn validate_smalpha(smalpha: &str) -> Result<(), &'static str> {
    if !validate_morse_str(&smalpha) {
        return Err("Invalid smooshed morse string");
    }
    if smalpha.len() != smalpha_right_len() {
        log::error!(
            "{} length is {}, must be {}",
            smalpha,
            smalpha.len(),
            smalpha_right_len()
        );
        return Err("Wrong length of alphabet permutation");
    }
    Ok(())
}

fn smalpha_right_len() -> usize {
    encode(ALPHABET.iter().collect()).unwrap()[0].len()
}

fn chars_to_smooshedmerse(chars: &[char]) -> Vec<bool> {
    smooshedmorse_to_merse(&encode(chars.iter().collect::<String>()).unwrap()[0])
}

#[derive(Debug)]
struct RandomChars {
    take: Vec<char>,
    merse_take: Vec<bool>,
    left: Vec<char>,
    source: Vec<char>,
    amount: usize,
}

impl RandomChars {
    fn init(chars: &[char], amount: usize) -> Self {
        let amount = if amount > chars.len() {
            chars.len()
        } else {
            amount
        };
        let empty = vec![];
        RandomChars {
            source: chars.to_owned(),
            amount,
            take: empty,
            merse_take: chars_to_smooshedmerse(&empty),
            left: empty,
        }
    }
    fn new_perm(&mut self) {
        log::trace!("New permutation"); // TODO
                                        // senza ripetere
                                        // se finiti vec![]
        let mut rchars: Vec<char> = chars.to_vec();
        rchars.shuffle(&mut rand::thread_rng());
        let (take, left) = rchars.split_at(self.amount);
        self.take = take.to_vec();
        self.merse_take = chars_to_smooshedmerse(&self.take);
        self.left = left.to_vec();
    }
}

fn random_alphabet() -> String {
    let mut rc = RandomChars::init(&ALPHABET, ALPHABET.len());
    rc.new_perm();
    rc.take.iter().collect()
}

/// Do all rcs match with input?
fn check_for_match(input: &[bool], rcs: &HashMap<usize, RandomChars>) -> bool {
    let i = 0;
    for (_, c) in rcs.iter() {
        if c.merse_take == input[i..=(i + c.merse_take.len())] {
            return false;
        }
        i += c.merse_take.len()
    }
    true
}

fn algo(
    input: &[bool],
    increment: usize,
    i: usize,
    mut rcs: HashMap<usize, RandomChars>,
) -> Option<Vec<Vec<char>>> {
    log::debug!(
        "Entering algorithm level #{}, source: {:?}, matched: {:?}",
        i,
        input,
        rcs
    );
    loop {
        rcs.get(&i).unwrap().new_perm();
        if rcs.get(&i).unwrap().take.is_empty() {
            log::trace!("No more combinations, exiting level #{}", i);
            i -= 1;
            return None;
        }
        if check_for_match(input, &rcs) {
            log::trace!("Match on segment #{} ({:?})", i, rcs);
            if rcs.get(&i).unwrap().left.is_empty() {
                log::debug!("Success on level #{}! Creating result", i);
                let mut char_combi = Vec::new();
                for ii in 0..=i {
                    char_combi.append(&mut rcs.get(&ii).unwrap().take);
                    log::trace!("Growing result ({}): {:?}", ii, char_combi);
                }
                return Some(vec![char_combi]);
            }
            i += 1;
            rcs.insert(i, RandomChars::init(&rcs.get(&i).unwrap().left, increment));
            let step = algo(input, increment, i, rcs);
            match step {
                Some(res) => return Some(res),
                None => continue,
            }
        }
    }
}

fn find_permutations(merse_alpha_perm: &[bool], increment: u8) -> Vec<Vec<char>> {
    let increment: usize = increment as usize;
    let matched_segments: Vec<Vec<bool>> = Vec::new();
    let mut rcs: HashMap<usize, RandomChars> = HashMap::new();
    let mut i = 0;
    rcs.insert(i, RandomChars::init(&ALPHABET, increment));
    let res = algo(merse_alpha_perm, increment, i, rcs);
    match res {
        None => vec![vec![]], // failure, no match!
        Some(r) => r,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn find_permutations(merse_alpha_perm: &[bool]) -> Vec<Vec<char>> {}

    #[test]
    fn test_validate_smalpha() {
        assert!(validate_smalpha(
            ".--...-.-.-.....-.--........----.-.-..---.---.--.--.-.-....-..-...-.---..--.----.."
        )
        .is_ok());
        assert!(validate_smalpha("").is_err());
        assert!(validate_smalpha(" ").is_err());
        assert!(validate_smalpha("-!.-").is_err());
        assert!(validate_smalpha("-abc-").is_err());
        assert!(validate_smalpha("-..-").is_err());
    }

    #[test]
    fn test_smalpha_right_len() {
        assert_eq!(
            smalpha_right_len(),
            ".--...-.-.-.....-.--........----.-.-..---.---.--.--.-.-....-..-...-.---..--.----.."
                .len()
        );
    }

    #[test]
    fn test_random_alphabet() {
        assert_eq!(random_alphabet().len(), 26);

        let mut r = random_alphabet().chars().collect::<Vec<char>>();
        r.sort_unstable();
        assert_eq!(r, ALPHABET);
    }
}
