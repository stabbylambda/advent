use common::{answer, read_input};
use nom::{
    character::complete::{i64 as nom_i64, newline},
    multi::separated_list0,
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Vec<i64>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list0(newline, nom_i64).parse(input);

    result.unwrap().1
}

fn mix(numbers: &Input, cycles: u64) -> Vec<i64> {
    // transform to pair of (idx, num)
    let mut numbers: Vec<(usize, &i64)> = numbers.iter().enumerate().collect();
    let size = numbers.len() as i64;

    for _cycle in 0..cycles {
        for og_index_to_find in 0..numbers.len() {
            // find where the index originally was
            let current_index = numbers
                .iter()
                .position(|&(og_index, _x)| og_index == og_index_to_find)
                .unwrap();

            // mod math our way around the list to get to the new insertion index
            let (_og_index, num) = numbers[current_index];
            let new_index = (current_index as i64 + num).rem_euclid(size - 1) as usize;

            // remove and re-insert the value
            let number = numbers.remove(current_index);
            numbers.insert(new_index, number);
        }
    }

    // transform back to just numbers
    numbers.iter().map(|&(_i, &x)| x).collect()
}

fn get_coordinates(numbers: &[i64]) -> i64 {
    let length = numbers.len();
    // find the 1, 2, and 3000th numbers from the current position of the 0 element
    let start = numbers.iter().position(|&x| x == 0).unwrap();
    (1..=3)
        .map(|x| numbers[(start + (x * 1000)) % length])
        .sum()
}

fn problem1(input: &Input) -> i64 {
    let result = mix(input, 1);
    get_coordinates(&result)
}

fn problem2(input: &Input) -> i64 {
    let decryption_key = 811_589_153;
    let multiplied: Vec<i64> = input.iter().map(|x| x * decryption_key).collect();
    let result = mix(&multiplied, 10);
    get_coordinates(&result)
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 3)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 1623178306)
    }
}
