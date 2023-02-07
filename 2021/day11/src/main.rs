use common::nom::single_digit;
use nom::{
    character::complete::newline,
    multi::{many1, separated_list1},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(input.clone());
    println!("problem 1 score: {score}");

    let score = problem2(input);
    println!("problem 2 score: {score}");
}
fn parse(input: &str) -> Vec<Vec<u32>> {
    let result: IResult<&str, Vec<Vec<u32>>> = separated_list1(newline, many1(single_digit))(input);

    result.unwrap().1
}

fn get_neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
    let x = x as i32;
    let y = y as i32;
    // find all neighbors
    (-1..2)
        .flat_map(|delta_y| {
            (-1..2)
                .filter_map(|delta_x| {
                    let new_y = y + delta_y;
                    let new_x = x + delta_x;

                    if new_y.is_negative()
                        || new_x.is_negative()
                        || new_y > 9
                        || new_x > 9
                        || new_y == y && new_x == x
                    {
                        None
                    } else {
                        Some((new_x as usize, new_y as usize))
                    }
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect()
}

fn step(input: &mut [Vec<u32>]) -> u32 {
    // first increment every octopus
    (0..10).for_each(|y| {
        for x in 0..10 {
            input[y][x] += 1;
        }
    });

    let mut total_flashes = 0;
    loop {
        let mut flashes = 0;
        for y in 0..10 {
            for x in 0..10 {
                if input[y][x] > 9 && input[y][x] != u32::MAX {
                    for (nx, ny) in get_neighbors(x, y) {
                        input[ny][nx] = input[ny][nx].saturating_add(1);
                    }

                    input[y][x] = u32::MAX;
                    flashes += 1;
                }
            }
        }
        total_flashes += dbg!(flashes);

        if flashes == 0 {
            break;
        }
    }

    //now reset if they've been flashed
    (0..10).for_each(|y| {
        for x in 0..10 {
            if input[y][x] == u32::MAX {
                input[y][x] = 0;
            }
        }
    });

    total_flashes
}

fn problem1(mut input: Vec<Vec<u32>>) -> u32 {
    let mut count = 0;
    for _s in 0..100 {
        let flashed = step(&mut input);

        count += flashed;
    }
    count
}

fn problem2(mut input: Vec<Vec<u32>>) -> u32 {
    let mut count = 0;
    loop {
        let flashed = step(&mut input);
        count += 1;
        if flashed == 100 {
            break;
        }
    }

    count
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(input);
        assert_eq!(result, 1656)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(input);
        assert_eq!(result, 195)
    }
}
