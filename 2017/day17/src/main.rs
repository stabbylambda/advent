fn main() {
    let input = 324;
    let answer = problem1(input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(input);
    println!("problem 2 answer: {answer}");
}

fn problem1(input: usize) -> u32 {
    let mut v: Vec<u32> = vec![0];
    let mut current = 0;
    let times = 2017;

    for n in 1..=times {
        current = ((current + input) % v.len()) + 1;
        v.insert(current, n);
    }

    let pos = v.iter().position(|x| *x == 2017).unwrap();
    v[pos + 1]
}

fn problem2(input: usize) -> usize {
    let mut current = 0;
    let mut value_after_zero = 0;

    for n in 1..=50_000_000 {
        current = ((current + input) % n) + 1;
        if current == 1 {
            value_after_zero = n;
        }
    }

    value_after_zero
}

#[cfg(test)]
mod test {
    use crate::{problem1, problem2};
    #[test]
    fn first() {
        let result = problem1(3);
        assert_eq!(result, 638)
    }

    #[test]
    fn second() {
        let result = problem2(3);
        assert_eq!(result, 1222153)
    }
}
