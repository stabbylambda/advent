use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Vec<Valley>;

fn parse(input: &str) -> Input {
    let image = map(
        separated_list1(newline, many1(alt((char('.'), char('#'))))),
        |x| Valley { valley: x },
    );
    let result: IResult<&str, Input> = separated_list1(tag("\n\n"), image)(input);

    result.unwrap().1
}

fn transpose(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = input[0].len();
    // transpose the nested vec so we can examine each char index
    let mut i_t: Vec<Vec<char>> = vec![vec![]; width];
    (0..width).for_each(|x| {
        (0..input.len()).for_each(|y| i_t[x].push(input[y][x]));
    });

    i_t
}

struct Valley {
    valley: Vec<Vec<char>>,
}

impl Valley {
    fn find_vertical_reflection(&self) -> Option<usize> {
        let image = transpose(&self.valley);
        let v = Valley { valley: image };
        v.find_reflection()
    }

    fn get_row_pair(&self, index: usize, offset: usize) -> Option<(&Vec<char>, &Vec<char>)> {
        let lower_idx = index.checked_sub(offset);
        let upper_idx = index.checked_add(offset + 1);

        let lower = lower_idx.and_then(|i| self.valley.get(i));
        let upper = upper_idx.and_then(|i| self.valley.get(i));

        lower.zip(upper)
    }

    fn find_reflection(&self) -> Option<usize> {
        // find all the pairs that equal each other so we know where we have to start
        let top_reflections: Vec<usize> = self
            .valley
            .iter()
            .enumerate()
            .tuple_windows()
            .filter_map(|((a_idx, a), (_b_idx, b))| (a == b).then_some(a_idx))
            .collect();

        // for each reflection row we found, start from there and compare outward
        top_reflections.into_iter().find_map(|t| {
            // start at 1, we don't need to compare the rows we just compared
            let mut offset = 1;
            // search until there are no more pairs to compare because we walked off the end of the grid
            while let Some((upper, lower)) = self.get_row_pair(t, offset) {
                // bail when we find something that doesn't match
                if upper != lower {
                    return None;
                }

                // keep going
                offset += 1;
            }

            Some(t + 1)
        })
    }

    fn find_horizontal_reflection(&self) -> Option<usize> {
        self.find_reflection().map(|x| x * 100)
    }

    fn find_any_reflection(&self) -> Option<usize> {
        self.find_horizontal_reflection()
            .or_else(|| self.find_vertical_reflection())
    }
}

fn problem1(input: &Input) -> usize {
    input.iter().filter_map(|x| x.find_any_reflection()).sum()
}

fn problem2(_input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn horizontal() {
        let input = include_str!("../horizontal.txt");
        let input = parse(input);
        let result = input[0].find_horizontal_reflection().unwrap();
        assert_eq!(result, 400)
    }

    #[test]
    fn vertical() {
        let input = include_str!("../vertical.txt");
        let input = parse(input);
        let result = input[0].find_vertical_reflection().unwrap();
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
        assert_eq!(result, 0)
    }
}
