use color_eyre::Report;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use tracing::info;

const WORDLIST: &str = "input/wordlist";

pub fn get_all_words(source_file: Option<&str>) -> Result<Vec<String>, Report> {
    let wordlist = source_file.unwrap_or(WORDLIST);
    let mut res: Vec<String> = Vec::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(wordlist) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            res.push(line?);
        }
    }
    info!("Wordlist loaded from {}", wordlist);
    Ok(res)
}

// fn all_words_iterator<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
//     if let Ok(lines) = read_lines(WORDLIST) {
//         for line in lines {
//             if let Ok(ip) = line {
//                 println!("{}", ip);
//             }
//         }
//     }
// }

/// Returns an Iterator to the Reader of the lines of the file.
/// The output is wrapped in a Result to allow matching on errors
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_words_length() {
        assert_eq!(get_all_words(None).unwrap().len(), 172823);
    }
}
