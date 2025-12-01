use std::ops::Div;

fn main() {
    let answer = problem1(HOUSES);
    println!("problem 1 answer: {answer}");

    let answer = problem2(HOUSES);
    println!("problem 2 answer: {answer}");
}

const HOUSES: u64 = 33_100_000;

fn divisor_sum<F>(num: u64, f: F) -> u64
where
    F: Fn(u64, u64) -> bool,
{
    // we only go up to the sqrt for divisors
    let cap = (num as f64).sqrt() as u64;

    (1..cap).fold(0, |acc, d| {
        if !num.is_multiple_of(d) {
            return acc;
        }

        let d1 = if f(num, d) { d } else { 0 };
        let d2 = if f(num, num / d) { num / d } else { 0 };

        acc + d1 + d2
    })
}

fn find_first_house<F>(houses: u64, f: F) -> u64
where
    F: Fn(u64, u64) -> bool,
{
    (1..)
        .find(|&house_number| divisor_sum(house_number, &f) >= houses)
        .unwrap()
}

fn problem1(input: u64) -> u64 {
    let houses = input / 10;
    find_first_house(houses, |_, _| true)
}

fn problem2(input: u64) -> u64 {
    let houses = input / 11;
    find_first_house(houses, |house, d| house.div(d) <= 50)
}

#[cfg(test)]
mod test {
    use crate::{problem1, problem2, HOUSES};
    #[test]
    fn first() {
        let result = problem1(HOUSES);
        assert_eq!(result, 776_160)
    }

    #[test]
    fn second() {
        let result = problem2(HOUSES);
        assert_eq!(result, 786_240)
    }
}
