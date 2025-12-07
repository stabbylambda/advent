use crate::{part1::problem1, part2::problem2};
use common::{answer, read_input};

pub mod part1;
pub mod part2;

fn main() {
    let input = read_input!();

    answer!(problem1(input));
    answer!(problem2(input));
}
