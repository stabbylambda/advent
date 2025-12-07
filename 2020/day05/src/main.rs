use common::{answer, read_input};
use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::map,
    multi::{count, separated_list1},
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Vec<Ticket>;

enum Direction {
    Front,
    Back,
    Left,
    Right,
}

struct Ticket {
    row: Vec<Direction>,
    seat: Vec<Direction>,
}

impl Ticket {
    fn row_number(&self) -> u32 {
        self.split(127, &self.row)
    }

    fn seat_number(&self) -> u32 {
        self.split(7, &self.seat)
    }

    fn seat_id(&self) -> u32 {
        self.row_number() * 8 + self.seat_number()
    }

    fn split(&self, mut high: u32, dirs: &[Direction]) -> u32 {
        let mut low: u32 = 0;
        for dir in dirs {
            let diff = (high.abs_diff(low) / 2) + 1;
            match dir {
                Direction::Front => high -= diff,
                Direction::Back => low += diff,

                Direction::Left => high -= diff,
                Direction::Right => low += diff,
            }
        }

        assert_eq!(low, high);

        low
    }
}

fn parse(input: &str) -> Input {
    let direction = |s| {
        alt((
            map(char('F'), |_| Direction::Front),
            map(char('B'), |_| Direction::Back),
            map(char('L'), |_| Direction::Left),
            map(char('R'), |_| Direction::Right),
        )).parse(s)
    };
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(
            (count(direction, 7), count(direction, 3)),
            |(row, seat)| Ticket { row, seat },
        ),
    ).parse(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> u32 {
    let seats: Vec<u32> = input.iter().map(|x| x.seat_id()).collect();
    *seats.iter().max().unwrap()
}

fn problem2(input: &Input) -> u32 {
    let seats: Vec<u32> = input.iter().map(|x| x.seat_id()).collect();

    let low = *seats.iter().min().unwrap();
    let high = *seats.iter().max().unwrap();

    (low..high)
        .find(|n| !seats.contains(n) && seats.contains(&(n - 1)) && seats.contains(&(n + 1)))
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 820)
    }
    #[test]
    fn row_seat_test() {
        let ticket = parse("FBFBBFFRLR");
        let ticket = ticket.first().unwrap();

        assert_eq!(ticket.row_number(), 44);
        assert_eq!(ticket.seat_number(), 5);
    }
}
