fn main() {
    let generator_a = Generator::basic(873, FACTOR_A);
    let generator_b = Generator::basic(583, FACTOR_B);

    let answer = problem1(generator_a, generator_b);
    println!("problem 1 answer: {answer}");

    let answer = problem2(generator_a, generator_b);
    println!("problem 1 answer: {answer}");
}

const FACTOR_A: u64 = 16807;
const FACTOR_B: u64 = 48271;

#[derive(Clone, Copy)]
struct Generator {
    current: u64,
    factor: u64,
    filter: Option<u64>,
}

impl Generator {
    fn basic(start: u64, factor: u64) -> Generator {
        Generator {
            current: start,
            factor,
            filter: None,
        }
    }

    fn filtered(&self, filter: u64) -> Generator {
        Generator {
            current: self.current,
            factor: self.factor,
            filter: Some(filter),
        }
    }
}

const DIVISOR: u64 = 2147483647;
impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = (self.current * self.factor) % DIVISOR;
        self.current = next;

        // yield only the last 16 bits
        let masked = next & 0xFFFF;

        match self.filter {
            None => Some(masked),
            Some(x) if next % x == 0 => Some(masked),
            // the next doesn't match the filter, so keep calculating
            _ => self.next(),
        }
    }
}

fn problem1(a: Generator, b: Generator) -> usize {
    a.zip(b).take(40_000_000).filter(|(a, b)| a == b).count()
}

fn problem2(a: Generator, b: Generator) -> usize {
    let a = a.filtered(4);
    let b = b.filtered(8);

    a.zip(b).take(5_000_000).filter(|(a, b)| a == b).count()
}

#[cfg(test)]
mod test {
    use crate::{problem1, problem2, Generator, FACTOR_A, FACTOR_B};
    #[test]
    fn first() {
        let a = Generator::basic(65, FACTOR_A);
        let b = Generator::basic(8921, FACTOR_B);
        let result = problem1(a, b);
        assert_eq!(result, 588)
    }

    #[test]
    fn second() {
        let a = Generator::basic(65, FACTOR_A);
        let b = Generator::basic(8921, FACTOR_B);
        let result = problem2(a, b);
        assert_eq!(result, 309)
    }
}
