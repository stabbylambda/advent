use common::answer;
use std::collections::HashMap;

use base64ct::{Base64, Encoding};
use md5::{Digest, Md5};

fn main() {
    let input = "cuanljph";

    answer!(problem1(input));
    answer!(problem2(input));
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct HashResult {
    index: u128,
    value: String,
    triple_char: char,
    quintuple_chars: Vec<char>,
}

impl PartialOrd for HashResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HashResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

fn hash(md5: &Md5, start: u128, stretch: bool) -> HashResult {
    for i in start.. {
        // todo: this is super slow, probably don't need to use the str versions of this though
        let mut hasher = md5.clone();
        hasher.update(i.to_string());

        let mut s = hasher.finalize();

        if stretch {
            for _n in 0..2016 {
                let mut m = Md5::new();
                m.update(s);
                s = m.finalize();
            }
        }

        let v = Base64::encode_string(&s);

        let first_triple = v
            .chars()
            .collect::<Vec<char>>()
            .windows(3)
            .find_map(|w| (w[0] == w[1] && w[1] == w[2]).then_some(w[0]));

        let quintuple_chars = v
            .chars()
            .collect::<Vec<char>>()
            .windows(5)
            .filter_map(|w| {
                (w[0] == w[1] && w[1] == w[2] && w[2] == w[3] && w[3] == w[4]).then_some(w[0])
            })
            .collect();

        if let Some(triple_char) = first_triple {
            return HashResult {
                index: i,
                value: v,
                triple_char,
                quintuple_chars,
            };
        }
    }

    unreachable!()
}

fn problem(input: &str, stretch: bool) -> u128 {
    let mut candidates: HashMap<char, Vec<HashResult>> = HashMap::new();
    let mut keys: Vec<HashResult> = vec![];
    let mut md5 = Md5::new();
    md5.update(input);

    let mut n = 0;
    while keys.len() <= 64 {
        let result = hash(&md5, n, stretch);

        // check all the chars that are in this quintuple
        for &quint in &result.quintuple_chars {
            let entry = candidates.entry(quint).or_default();

            // get all the candidates that are less than 1000 hashes old
            let confirmed: Vec<HashResult> = entry
                .iter()
                .filter(|x| x.index.abs_diff(result.index) <= 1000)
                .cloned()
                .collect();

            confirmed.iter().for_each(|x| {
                println!(
                    "{} - {} ({}) confirmed by {} - {} ({:?}) | Delta: {} ",
                    x.index,
                    x.value,
                    x.triple_char,
                    result.index,
                    result.value,
                    quint,
                    x.index.abs_diff(result.index)
                )
            });
            keys.extend(confirmed);

            entry.clear();
        }

        // all hashes with a triple are candidates, so we need to start tracking this one
        candidates
            .entry(result.triple_char)
            .or_default()
            .push(result.clone());

        // find the next hash
        n = result.index + 1;
    }

    // this is dumb, because we're just extending the array, we can wind up with more than 64 hashes
    // we need to sort by index and then take the 64th one
    keys.sort();
    keys[63].index
}

fn problem1(input: &str) -> u128 {
    problem(input, false)
}

fn problem2(input: &str) -> u128 {
    problem(input, true)
}

#[cfg(test)]
mod test {
    use crate::{problem1, problem2};
    #[test]
    #[ignore = "too slow"]
    fn first() {
        let input = "abc";
        let result = problem1(input);
        assert_eq!(result, 22728)
    }

    #[test]
    #[ignore = "too slow"]
    fn second() {
        let input = "abc";
        let result = problem2(input);
        assert_eq!(result, 22551)
    }
}
