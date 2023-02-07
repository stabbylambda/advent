use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let input: Vec<&str> = input.lines().collect();

    let score = problem1(&input[..]);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

fn is_nice1(input: &str) -> bool {
    fn is_vowel(c: &char) -> bool {
        c == &'a' || c == &'e' || c == &'i' || c == &'o' || c == &'u'
    }

    fn is_forbidden(cs: &[char]) -> bool {
        let ab = cs[0] == 'a' && cs[1] == 'b';
        let cd = cs[0] == 'c' && cs[1] == 'd';
        let pq = cs[0] == 'p' && cs[1] == 'q';
        let xy = cs[0] == 'x' && cs[1] == 'y';

        ab || cd || pq || xy
    }

    let cs: Vec<char> = input.chars().collect();
    let vowel_count = cs.iter().filter(|x| is_vowel(x)).count();
    let has_forbidden = cs.windows(2).any(is_forbidden);
    let has_double = cs.windows(2).any(|x| x[0] == x[1]);

    vowel_count >= 3 && has_double && !has_forbidden
}

fn is_nice2(input: &str) -> bool {
    let cs: Vec<char> = input.chars().collect();

    let mut seen: HashMap<(char, char), Vec<usize>> = HashMap::new();

    for (idx, cs) in cs.windows(2).enumerate() {
        let cs = (cs[0], cs[1]);
        seen.entry(cs)
            .and_modify(|v| v.push(idx))
            .or_insert_with(|| vec![idx]);
    }

    let has_multiple = seen
        .values()
        .filter(|x| x.len() > 1)
        .any(|idxs| idxs.windows(2).any(|x| x[0].abs_diff(x[1]) > 1));

    let has_split_repeat = cs.windows(3).any(|x| x[0] == x[2]);

    has_split_repeat && has_multiple
}

fn problem1(input: &[&str]) -> usize {
    input.iter().filter(|x| is_nice1(x)).count()
}

fn problem2(input: &[&str]) -> usize {
    input.iter().filter(|x| is_nice2(x)).count()
}

#[cfg(test)]
mod test {
    use crate::{is_nice1, is_nice2};

    #[test]
    fn test_is_nice1() {
        assert!(is_nice1("aaa"));
        assert!(is_nice1("ugknbfddgicrmopn"));
        assert!(!is_nice1("jchzalrnumimnmhp"));
        assert!(!is_nice1("haegwjzuvuyypxyu"));
        assert!(!is_nice1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_is_nice2() {
        assert!(is_nice2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice2("xxyxx"));
        assert!(!is_nice2("uurcxstgmygtbstg"));
        assert!(!is_nice2("ieodomkazucvgmuy"));
    }
}
