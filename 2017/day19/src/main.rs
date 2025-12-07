use common::{answer, read_input};

fn main() {
    let input = read_input!();
    let input = parse(input);

    let (answer1, answer2) = problem1(&input);
    answer!(answer1);
    answer!(answer2);
}

type Input = Vec<Vec<char>>;

fn parse(input: &str) -> Input {
    input.lines().map(|l| l.chars().collect()).collect()
}

enum Direction {
    North,
    South,
    West,
    East,
}

fn problem1(input: &Input) -> (String, u32) {
    // find the first space and start going down
    let mut y = 0;
    let mut x = input[y].iter().position(|x| *x != ' ').unwrap();
    let mut dir = Direction::South;

    // we're going to accumulate the letters and the number of steps
    let mut letters = vec![];
    let mut steps = 0;

    let grid = common::grid::Grid::new(input.clone());

    loop {
        match *grid.get((x, y)).data {
            // we reached the end of the line
            ' ' => break,
            // plusses are change in direction
            '+' => {
                let neighbors = grid.get((x, y)).neighbors();
                let north = neighbors
                    .north
                    .filter(|x| !x.data.is_whitespace())
                    .is_some();
                let south = neighbors
                    .south
                    .filter(|x| !x.data.is_whitespace())
                    .is_some();
                let east = neighbors.east.filter(|x| !x.data.is_whitespace()).is_some();
                let west = neighbors.west.filter(|x| !x.data.is_whitespace()).is_some();

                // switch directions, we can only turn left or right
                dir = match dir {
                    Direction::North if west => Direction::West,
                    Direction::North if east => Direction::East,
                    Direction::South if west => Direction::West,
                    Direction::South if east => Direction::East,
                    Direction::West if north => Direction::North,
                    Direction::West if south => Direction::South,
                    Direction::East if north => Direction::North,
                    Direction::East if south => Direction::South,

                    _ => unreachable!(),
                }
            }
            // if we ran across an alphabet character, keep track of it
            c if c.is_alphabetic() => letters.push(c),
            // anything else is nothing
            _ => {}
        }

        steps += 1;

        // keep going in the current direction
        (x, y) = match dir {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
            Direction::East => (x + 1, y),
        }
    }

    (letters.iter().collect(), steps)
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let (result1, result2) = problem1(&input);
        assert_eq!(result1, "ABCDEF");
        assert_eq!(result2, 38);
    }
}
