//! SmooshedMorse challenge 2
//! Given a smooshed Morse code encoding of a permutation of the alphabet, find the permutation it
//! encodes, or any other permutation that produces the same encoding (in general there will be
//! more than one)
//! https://www.reddit.com/r/dailyprogrammer/comments/cn6gz5/20190807_challenge_380_intermediate_smooshed/

use crate::encode::encode;
use crate::merses::smooshedmorse_to_merse;
use crate::morses::validate_morse_str;
use crate::morses::ALPHABET;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

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
        .into_iter()
        .map(|alphchars| alphchars.into_iter().collect::<String>())
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
struct SegmentChars {
    take: Vec<char>,
    merse_take: Vec<bool>,
    left: Vec<char>,
    source: Vec<char>,
    amount: usize,
    permutations: Vec<Vec<char>>,
}

impl SegmentChars {
    fn init(chars: &[char], amount: usize) -> Self {
        let amount = if amount > chars.len() {
            chars.len()
        } else {
            amount
        };
        let permutations = chars
            .to_owned()
            .into_iter()
            .permutations(amount)
            .into_iter()
            .collect();
        SegmentChars {
            source: chars.to_owned(),
            amount,
            take: vec![],
            merse_take: chars_to_smooshedmerse(&[]),
            left: vec![],
            permutations,
        }
    }
    fn get_left(&self, source: &[char], taken: &[char]) -> Vec<char> {
        // source - taken
        let taken_set: HashSet<_> = taken.iter().collect();
        let difference: Vec<char> = source
            .to_owned()
            .into_iter()
            .filter(|item| !taken_set.contains(item))
            .collect();
        difference
    }
    fn new_perm(&mut self) {
        log::trace!("New permutation");
        self.take = match self.permutations.pop() {
            Some(p) => p,
            None => {
                log::trace!("No more permutations");
                vec![]
            }
        };
        self.merse_take = chars_to_smooshedmerse(&self.take);
        self.left = self.get_left(&self.source, &self.take);
    }
}

fn random_alphabet() -> String {
    let mut rc = SegmentChars::init(&ALPHABET, ALPHABET.len());
    rc.new_perm();
    rc.take.into_iter().collect()
}

/// Do all segchs match with input?
fn check_for_match(input: &[bool], segchs: &HashMap<usize, SegmentChars>) -> bool {
    let mut i = 0;
    for (_, c) in segchs.iter() {
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
    mut i: usize,
    segchs: &mut HashMap<usize, SegmentChars>,
) -> (usize, Option<Vec<Vec<char>>>) {
    log::debug!(
        "Entering algorithm level #{}, source: {:?}, {:?}",
        &i,
        &input,
        &segchs
    );
    loop {
        segchs.get_mut(&i).unwrap().new_perm();
        if segchs.get(&i).unwrap().take.is_empty() {
            log::trace!("No more combinations, exiting level #{}", i);
            i -= 1;
            return (i, None);
        }
        if check_for_match(input, segchs) {
            log::trace!("Match on segment #{})", i);
            if segchs.get(&i).unwrap().left.is_empty() {
                log::debug!("Success on level #{}! Creating result", i);
                let mut char_combi = Vec::new();
                for ii in 0..=i {
                    let matching_seg: &mut SegmentChars = segchs.get_mut(&ii).unwrap();
                    char_combi.append(&mut matching_seg.take);
                    log::trace!("Growing result ({}): {:?}", ii, char_combi);
                }
                return (i, Some(vec![char_combi]));
            }
            i += 1;
            let segch_new = SegmentChars::init(&segchs.get(&i).unwrap().left, increment);
            segchs.insert(i, segch_new);
            let (i, step) = algo(input, increment, i, segchs);
            match step {
                Some(res) => return (i, Some(res)),
                None => continue,
            }
        }
    }
}

fn find_permutations(merse_alpha_perm: &[bool], increment: u8) -> Vec<Vec<char>> {
    let increment: usize = increment as usize;
    let mut segchs: HashMap<usize, SegmentChars> = HashMap::new();
    let i = 0;
    let segch0 = SegmentChars::init(&ALPHABET, increment);
    segchs.insert(i, segch0);
    let (_i, res) = algo(merse_alpha_perm, increment, i, &mut segchs);
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
