// Advent of Code Day 2 solutions
// This one was a bit messy. Lots of unchecked vectors and I'm likely assigning
// more variables than I have to in every loop. But it works, it's quick, and
// I have a busy week ahead of me. I'd like to get these first few days done
// quickly before I have to spend a few days working on the harder problems as
// they come along.

use regex::Regex;
use std::fs;

fn main() {
    let password_format = Regex::new(r"([0-9]{1,3})-([0-9]{1,3}) ([a-z]): ([a-z]+)")
        .expect("The regular expression format was invalid.");
    let input_file = fs::read_to_string("input.txt").expect("Couldn't read the file!");

    let mut first_criteria_passwords: usize = 0;
    let mut second_criteria_passwords: usize = 0;

    for password_info in password_format.captures_iter(&input_file) {
        let lower_bound: usize = password_info[1]
            .parse()
            .expect("The lower bound in this line was not a 1-3 digit integer.");
        let upper_bound: usize = password_info[2]
            .parse()
            .expect("The upper bound in this line was not a 1-3 digit integer.");

        let match_count = password_info[4].matches(&password_info[3]).count();

        if lower_bound <= match_count && match_count <= upper_bound {
            first_criteria_passwords += 1;
        }

        let chars: Vec<char> = password_info[4].chars().collect();
        let match_char: char = password_info[3]
            .chars()
            .nth(0)
            .expect("The character to match was not present.");

        let first_char_matched: bool = chars[lower_bound - 1] == match_char;
        let second_char_matched: bool = chars[upper_bound - 1] == match_char;

        if (first_char_matched || second_char_matched)
            && !(first_char_matched && second_char_matched)
        {
            second_criteria_passwords += 1;
        }
    }

    println!(
        "{} passwords matched the first criteria, and {} matched the second.",
        first_criteria_passwords, second_criteria_passwords
    );
}
