use advent_2015_09::{parse, problem1};
use common::{answer, read_input};

fn main() {
    let input = read_input!();
    let input = parse(input);

    let (min, max) = problem1(&input);
    answer!(min);
    answer!(max);
}
