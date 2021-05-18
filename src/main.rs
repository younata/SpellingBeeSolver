use structopt::StructOpt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "w", long = "words", parse(from_os_str), default_value = "/usr/share/dict/words")]
    words_file: std::path::PathBuf,
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
    primary_character: char,
    other_characters: String,
}

fn main() {
    let args = Cli::from_args();
    if args.verbose {
        println!("Searching for solutions...");
        println!("");
    }

    let mut allowed_string = args.other_characters;
    allowed_string.push(args.primary_character);

    if let Ok(lines) = read_lines(args.words_file) {
        for line in lines {
            if let Ok(ip) = line {
                if word_matches(&ip, args.primary_character, &allowed_string) {
                    println!("{}", ip);
                }
            }
        }
    }
    if args.verbose {
        println!("");
        println!("Done!");
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn word_matches(word: &String, primary_character: char, allowed_characters: &String) -> bool {
    if word.len() < 4 {
        return false;
    }
    if word.contains(primary_character) == false {
        return false;
    }
    for char in word.chars() {
        if allowed_characters.contains(char) {
            continue;
        }
        return false;
    }
    return true;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn word_matches_too_short() {
        assert_eq!(word_matches(&"foo".to_string(), 'f', &"fo".to_string()), false);
    }

    #[test]
    fn word_matches_does_not_contain_primary_character() {
        assert_eq!(word_matches(&"foobar".to_string(), 't', &"fotbar".to_string()), false);
    }

    #[test]
    fn word_matches_contains_disallowed_characters() {
        assert_eq!(word_matches(&"foobar".to_string(), 'o', &"fotba".to_string()), false);
        // the 'r' is disallowed.
    }

    #[test]
    fn word_matches_passes() {
        assert_eq!(word_matches(&"foobar".to_string(), 'o', &"fobar".to_string()), true);
    }
}
