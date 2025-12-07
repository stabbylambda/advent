use common::{answer, grid::Grid, nom::parse_grid, read_input};
use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, combinator::map,
    multi::separated_list1, IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Vec<Grid<bool>>;

fn parse(input: &str) -> Input {
    let image = parse_grid(alt((map(char('.'), |_| false), map(char('#'), |_| true))));
    let result: IResult<&str, Input> = separated_list1(tag("\n\n"), image).parse(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

impl Reflection {
    fn to_score(self) -> usize {
        match self {
            Reflection::Horizontal(x) => 100 * x,
            Reflection::Vertical(x) => x,
        }
    }
}

fn get_row_pair(map: &Grid<bool>, index: usize, offset: usize) -> Option<(&Vec<bool>, &Vec<bool>)> {
    let lower_idx = index.checked_sub(offset);
    let upper_idx = index.checked_add(offset + 1);

    let lower = lower_idx.and_then(|i| map.points.get(i));
    let upper = upper_idx.and_then(|i| map.points.get(i));

    lower.zip(upper)
}

fn find_reflections(map: &Grid<bool>, f: impl Fn(usize) -> Reflection) -> Vec<Reflection> {
    (0..map.height)
        .filter_map(|i| {
            // check this row and the next
            let (a, b) = get_row_pair(map, i, 0)?;

            // if they're not equal, bail
            if a != b {
                return None;
            }

            // start at 1, we don't need to compare the rows we just compared
            let mut offset = 1;

            // search until there are no more pairs to compare because we walked off the end of the grid
            while let Some((upper, lower)) = get_row_pair(map, i, offset) {
                // bail when we find something that doesn't match
                if upper != lower {
                    return None;
                }

                // keep going
                offset += 1;
            }

            Some(f(i + 1))
        })
        .collect_vec()
}

fn find_all_reflections(map: &Grid<bool>) -> Vec<Reflection> {
    vec![
        find_reflections(map, Reflection::Horizontal),
        find_reflections(&map.transpose(), Reflection::Vertical),
    ]
    .into_iter()
    .flatten()
    .collect_vec()
}

/** For each square in the map, create a clone of the map where it's flipped */
fn get_all_smudges(map: &Grid<bool>) -> Vec<Grid<bool>> {
    map.iter()
        .map(|x| {
            let mut m = map.clone();
            m.set(x.coords, !x.data);
            m
        })
        .collect_vec()
}

fn problem1(input: &Input) -> usize {
    input
        .iter()
        .flat_map(find_all_reflections)
        .map(|r| r.to_score())
        .sum()
}

fn problem2(input: &Input) -> usize {
    input
        .iter()
        .flat_map(|v| {
            let r = *find_all_reflections(v).first().unwrap();
            let smudges = get_all_smudges(v);

            smudges
                .iter()
                .flat_map(find_all_reflections)
                .filter(|r1| r != *r1)
                .unique()
                .collect_vec()
        })
        .map(|x| x.to_score())
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{find_reflections, parse, problem1, problem2, Reflection};
    #[test]
    fn horizontal() {
        let input = include_str!("../horizontal.txt");
        let input = parse(input);
        let result = find_reflections(&input[0], Reflection::Horizontal)
            .first()
            .unwrap()
            .to_score();
        assert_eq!(result, 400)
    }

    #[test]
    fn vertical() {
        let input = include_str!("../vertical.txt");
        let input = parse(input);
        let result = find_reflections(&input[0].transpose(), Reflection::Vertical)
            .first()
            .unwrap()
            .to_score();
        assert_eq!(result, 5)
    }

    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 405)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 400)
    }
}
