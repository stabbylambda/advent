use cavemap::{CaveMap, Path, Tile};
use common::nom::coord;
use nom::{
    bytes::complete::tag, character::complete::newline, combinator::map, multi::separated_list1,
    IResult,
};

pub mod cavemap;

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Vec<Path>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> =
        separated_list1(newline, map(separated_list1(tag(" -> "), coord), Path::new))(input);

    result.unwrap().1
}

#[derive(PartialEq, Eq)]
enum SandResult {
    Clogged,
    Abyss,
    Settled,
}

fn simulate_sand(input: &mut CaveMap) -> SandResult {
    // every sand particle starts at the source
    let (mut x, mut y) = input.source;

    // check if we're off the left or right edge or below the lowest rock
    while 0 < x && x < input.map.width && y < input.map.height - 1 {
        // check the next square down
        let down = input.map.get((x, y + 1)).data;
        let diag_left = input.map.get((x - 1, y + 1)).data;
        let diag_right = input.map.get((x + 1, y + 1)).data;

        if down == &Tile::Air {
            y += 1;
        } else if diag_left == &Tile::Air {
            // move left
            x -= 1;
            y += 1;
        } else if diag_right == &Tile::Air {
            // move right
            x += 1;
            y += 1;
        } else {
            // we can't move down, diagonal left, or diagonal right so we settle here
            input.map.set((x, y), Tile::Sand);
            return if input.map.get(input.source).data == &Tile::Sand {
                SandResult::Clogged
            } else {
                SandResult::Settled
            };
        }
    }
    // we're off the map, so return Abyss
    SandResult::Abyss
}

fn problem1(input: &Input) -> u32 {
    let mut grains = 0;
    let mut map = CaveMap::new(input, false);
    while simulate_sand(&mut map) != SandResult::Abyss {
        grains += 1;
    }

    grains
}

fn problem2(input: &Input) -> u32 {
    let mut grains = 0;
    let mut map = CaveMap::new(input, true);
    while simulate_sand(&mut map) != SandResult::Clogged {
        grains += 1;
    }

    grains + 1
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 24)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 93)
    }
}
