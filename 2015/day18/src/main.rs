use ndarray::{Array3, Axis};
use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list1},
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input, 100);

    let mut lights = input.clone();
    let answer = problem1(&mut lights, 100);
    println!("problem 1 answer: {answer}");

    let mut lights = input;
    let answer = problem2(&mut lights, 100);
    println!("problem 2 answer: {answer}");
}

type Input = Array3<bool>;

fn to_array(steps: usize, grid: Vec<Vec<bool>>) -> Array3<bool> {
    let height = grid.len();
    let width = grid.iter().map(|x| x.len()).max().unwrap();

    let mut new_grid = Array3::from_elem((steps + 1, height, width), false);

    // pack the vec vec into an ndarray
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            new_grid[[0, y, x]] = *cell;
        }
    }

    new_grid
}

fn parse(input: &str, steps: usize) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list1(
            newline,
            many1(alt((map(char('.'), |_| false), map(char('#'), |_| true)))),
        ),
        |x| to_array(steps, x),
    ).parse(input);

    result.unwrap().1
}

fn problem1(lights: &mut Input, steps: usize) -> usize {
    let max_y = lights.len_of(Axis(1));
    let max_x = lights.len_of(Axis(2));

    for n in 0usize..steps {
        for y in 0..max_y {
            for x in 0..max_x {
                let current = lights[[n, y, x]];

                let neighbors = [
                    y > 0 && x > 0 && lights[[n, y - 1, x - 1]],
                    y > 0 && lights[[n, y - 1, x]],
                    y > 0 && x < max_x - 1 && lights[[n, y - 1, x + 1]],
                    x > 0 && lights[[n, y, x - 1]],
                    x < max_x - 1 && lights[[n, y, x + 1]],
                    y < max_y - 1 && x > 0 && lights[[n, y + 1, x - 1]],
                    y < max_y - 1 && lights[[n, y + 1, x]],
                    y < max_y - 1 && x < max_x - 1 && lights[[n, y + 1, x + 1]],
                ];

                let on_count = neighbors.iter().filter(|x| **x).count();

                let new_light = match current {
                    true => on_count == 2 || on_count == 3,
                    false => on_count == 3,
                };

                lights[[n + 1, y, x]] = new_light;
            }
        }
    }

    lights
        .index_axis(Axis(0), steps)
        .iter()
        .filter(|x| **x)
        .count()
}

fn problem2(lights: &mut Input, steps: usize) -> usize {
    let max_y = lights.len_of(Axis(1)) - 1;
    let max_x = lights.len_of(Axis(2)) - 1;

    // prep the four corners
    lights[[0, 0, 0]] = true;
    lights[[0, 0, max_x]] = true;
    lights[[0, max_y, 0]] = true;
    lights[[0, max_y, max_x]] = true;

    for n in 0usize..steps {
        for y in 0..=max_y {
            for x in 0..=max_x {
                let current = lights[[n, y, x]];

                let neighbors = [
                    y > 0 && x > 0 && lights[[n, y - 1, x - 1]],
                    y > 0 && lights[[n, y - 1, x]],
                    y > 0 && x < max_x && lights[[n, y - 1, x + 1]],
                    x > 0 && lights[[n, y, x - 1]],
                    x < max_x && lights[[n, y, x + 1]],
                    y < max_y && x > 0 && lights[[n, y + 1, x - 1]],
                    y < max_y && lights[[n, y + 1, x]],
                    y < max_y && x < max_x && lights[[n, y + 1, x + 1]],
                ];

                let on_count = neighbors.iter().filter(|x| **x).count();

                let new_light = match current {
                    true => on_count == 2 || on_count == 3,
                    false => on_count == 3,
                };

                lights[[n + 1, y, x]] = match (y, x) {
                    (0, 0) => true,
                    (0, x) if x == max_x => true,
                    (y, 0) if y == max_y => true,
                    (y, x) if x == max_x && y == max_y => true,
                    _ => new_light,
                };
            }
        }
    }

    lights
        .index_axis(Axis(0), steps)
        .iter()
        .filter(|x| **x)
        .count()
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let mut input = parse(input, 4);
        let result = problem1(&mut input, 4);
        assert_eq!(result, 4)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let mut input = parse(input, 5);
        let result = problem2(&mut input, 5);
        assert_eq!(result, 17)
    }
}
