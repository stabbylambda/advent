use common::get_raw_input;
use itertools::Itertools;

fn main() {
    let input = get_raw_input();

    let answer1 = problem(&input);
    println!("problem 1 answer: {answer1}");

    let answer2 = problem(&answer1);
    println!("problem 2 answer: {answer2}");
}

const ALLOWED_LETTERS: [char; 23] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v',
    'w', 'x', 'y', 'z',
];

fn increment_password(s: &str) -> String {
    match s.chars().last() {
        // we're at the end of the string, just return blank
        None => "".to_string(),
        Some(c) => {
            let idx = ALLOWED_LETTERS.iter().position(|x| *x == c).unwrap();
            let remaining = &s[0..s.len() - 1];

            let (remaining, next) = match ALLOWED_LETTERS.get(idx + 1) {
                None => (increment_password(remaining), 'a'),
                Some(next) => (remaining.to_string(), *next),
            };

            format!("{remaining}{next}")
        }
    }
}

fn is_valid_password(s: &str) -> bool {
    let mut has_run = false;
    let mut pair_count = 0;
    let mut char_iter = s.bytes().tuple_windows::<(_, _, _)>();

    while let Some((c1, c2, c3)) = char_iter.next() {
        // check if this window is a run
        has_run = has_run || (c1 + 1 == c2 && c2 + 1 == c3);

        // check if either of these are a matched pair
        if c1 == c2 || c2 == c3 {
            pair_count += 1;
            if pair_count == 2 {
                break;
            } else {
                // don't consider c2 for another overlapping window
                char_iter.next();
            }
        }
    }

    has_run && pair_count >= 2
}

fn find_next_password(s: &str) -> String {
    let mut next: String = s.to_owned();
    loop {
        next = increment_password(&next);
        if is_valid_password(&next) {
            return next;
        }
    }
}

fn problem(input: &str) -> String {
    find_next_password(input)
}

#[cfg(test)]
mod test {

    use crate::{is_valid_password, problem};
    #[test]
    fn valid() {
        assert!(!is_valid_password("abbceffg"));
        assert!(!is_valid_password("abbcegjk"));

        assert!(is_valid_password("abcdffaa"));
        assert!(is_valid_password("ghjaabcc"));
    }
    #[test]
    fn first() {
        let input = "abcdefgh";
        let result = problem(input);
        assert_eq!(result, "abcdffaa")
    }
}
