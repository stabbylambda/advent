use std::fmt::Display;

fn main() {
    let input = common::read_input!();

    let answer = problem1(input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(input);
    println!("problem 2 answer: {answer}");
}

#[derive(Clone, Copy, Debug)]
struct Unit {
    letter: char,
    polarity: bool,
}

impl Unit {
    fn new(c: char) -> Unit {
        Unit {
            letter: c.to_ascii_lowercase(),
            polarity: c.is_lowercase(),
        }
    }

    fn destroys(&self, other: &Unit) -> bool {
        self.letter == other.letter && self.polarity != other.polarity
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = if self.polarity {
            self.letter
        } else {
            self.letter.to_ascii_uppercase()
        };
        write!(f, "{x}")
    }
}

#[derive(Clone)]
struct Polymer {
    units: Vec<Unit>,
}

impl Polymer {
    fn new(s: &str) -> Polymer {
        Polymer {
            units: s.chars().map(Unit::new).collect(),
        }
    }

    fn len(&self) -> usize {
        self.units.len()
    }

    fn without(&self, c: char) -> Polymer {
        let units = self
            .units
            .iter()
            .filter(|u| u.letter != c)
            .cloned()
            .collect();
        Polymer { units }
    }

    fn reduce(&self) -> Polymer {
        let mut polymer = self.clone();
        while let Some(new_polymer) = polymer.reduce_step() {
            polymer = new_polymer;
        }

        polymer
    }

    fn reduce_step(&self) -> Option<Polymer> {
        let mut units: Vec<Unit> = vec![];
        let mut n = 0;
        let mut destroyed = false;

        while n < self.len() {
            if n + 1 < self.len() && self.units[n].destroys(&self.units[n + 1]) {
                // these units destroyed each other, move past the next one
                destroyed = true;
                n += 1;
            } else {
                units.push(self.units[n]);
            }
            n += 1;
        }

        destroyed.then_some(Polymer { units })
    }
}

impl Display for Polymer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .units
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("");
        write!(f, "{s}")
    }
}

fn problem1(input: &str) -> usize {
    Polymer::new(input).reduce().len()
}

fn problem2(input: &str) -> usize {
    let original = Polymer::new(input);
    let mut min = usize::MAX;

    for c in 'a'..='z' {
        min = min.min(original.without(c).reduce().len())
    }

    min
}

#[cfg(test)]
mod test {
    use crate::{problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let result = problem1(input);
        assert_eq!(result, 10)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let result = problem2(input);
        assert_eq!(result, 4)
    }
}
