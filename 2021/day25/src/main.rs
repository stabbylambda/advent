use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list1},
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");
    println!("Merry Christmas!");
}

type Input = Vec<Vec<Tile>>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        many1(alt((
            map(char('v'), |_| Tile::South),
            map(char('>'), |_| Tile::East),
            map(char('.'), |_| Tile::Empty),
        ))),
    ).parse(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    East,
    South,
    Empty,
}

fn problem1(input: &Input) -> u32 {
    let height = input.len();
    let width = input[0].len();

    let mut grid = input.clone();

    for tick in 1.. {
        let mut next_east = grid.clone();

        // first deal with east
        for (y, row) in grid.iter().enumerate() {
            for (x, t) in row.iter().enumerate() {
                if t == &Tile::East {
                    //always look at the original grid
                    let neighbor = grid[y][(x + 1) % width];
                    if neighbor == Tile::Empty {
                        next_east[y][(x + 1) % width] = Tile::East;
                        next_east[y][x] = Tile::Empty;
                    }
                }
            }
        }

        let mut next_south = next_east.clone();

        //now deal with south
        for (y, row) in next_east.iter().enumerate() {
            for (x, t) in row.iter().enumerate() {
                if t == &Tile::South {
                    let neighbor = next_east[(y + 1) % height][x];
                    if neighbor == Tile::Empty {
                        next_south[(y + 1) % height][x] = Tile::South;
                        next_south[y][x] = Tile::Empty;
                    }
                }
            }
        }

        if next_south == grid {
            return tick;
        }

        grid = next_south;
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 58)
    }
}
