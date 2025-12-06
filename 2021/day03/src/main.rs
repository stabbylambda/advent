use std::str::Lines;

fn main() {
    let input = common::read_input!();
    let data = Data::new(input.lines());

    let first = first_problem(&data);
    println!("first result: {first}");

    let second = second_problem(&data);
    println!("second result: {second}");
}

fn get_frequencies(length: usize, numbers: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mut zeros = vec![0; length];
    let mut ones = vec![0; length];

    for x in numbers {
        for k in (0..length).rev() {
            // gotta reverse index the vector because bit shifts are backwards
            let idx = length - k - 1;
            let bit = x & (1 << k);
            if bit != 0 {
                ones[idx] += 1;
            } else {
                zeros[idx] += 1;
            }
        }
    }

    (zeros, ones)
}

struct Data {
    length: usize,
    mask: i32,
    numbers: Vec<i32>,
}

impl Data {
    fn new(input: Lines) -> Data {
        let length = input.clone().next().unwrap().len();
        let mask = !(i32::MAX << length);

        let numbers: Vec<i32> = input.map(|s| i32::from_str_radix(s, 2).unwrap()).collect();

        Data {
            length,
            mask,
            numbers,
        }
    }
}

fn get_gamma(length: usize, numbers: &[i32]) -> i32 {
    let (zeros, ones) = get_frequencies(length, numbers);

    let gamma: String = zeros
        .iter()
        .zip(ones.iter())
        .map(|(z, o)| if z > o { '0' } else { '1' })
        .collect();

    i32::from_str_radix(&gamma, 2).unwrap()
}

fn first_problem(data: &Data) -> i32 {
    let gamma = get_gamma(data.length, &data.numbers);
    let epsilon = data.mask ^ gamma;

    gamma * epsilon
}

fn matching_values(k: usize, comparator: i32, numbers: &[i32]) -> Vec<i32> {
    let cmp_bit = comparator & (1 << k);
    numbers
        .iter()
        .filter(|x| {
            let x_bit = *x & (1 << k);
            cmp_bit == x_bit
        })
        .map(|x| x.to_owned())
        .collect()
}

fn get_rating<F>(data: &Data, f: F) -> i32
where
    F: Fn(&Data, &[i32]) -> i32,
{
    (0..data.length)
        .rev()
        .fold(data.numbers.clone(), |current, k| {
            if current.len() == 1 {
                return current;
            }

            let comparator = f(data, &current);
            matching_values(k, comparator, &current)
        })
        .first()
        .unwrap()
        .to_owned()
}

fn second_problem(data: &Data) -> i32 {
    let oxygen_generator_rating = get_rating(data, |data, current| get_gamma(data.length, current));

    let co2_scrubber_rating = get_rating(data, |data, current| {
        let gamma = get_gamma(data.length, current);
        data.mask ^ gamma
    });

    oxygen_generator_rating * co2_scrubber_rating
}

#[test]
fn first() {
    let input = include_str!("../test.txt");
    let data = Data::new(input.lines());
    let result = first_problem(&data);
    assert_eq!(198, result);
}

#[test]
fn second() {
    let input = include_str!("../test.txt");
    let data = Data::new(input.lines());
    let result = second_problem(&data);
    assert_eq!(230, result);
}
