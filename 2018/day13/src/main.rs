use nom::{
    character::complete::{newline, not_line_ending},
    combinator::map,
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let (x, y) = problem1(&input);
    println!("problem 1 answer: {x},{y}");

    let (x, y) = problem2(&input);
    println!("problem 2 answer: {x},{y}");
}

type Input = Vec<Vec<char>>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(not_line_ending, |x: &str| x.chars().into_iter().collect()),
    )(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cart {
    x: usize,
    y: usize,
    direction: Direction,
    next_turn: Turn,
}

impl Cart {
    fn new(x: usize, y: usize, direction: Direction) -> Cart {
        Cart {
            x,
            y,
            direction,
            next_turn: Turn::Left,
        }
    }

    fn next(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        };
    }
    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        };
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };
    }

    fn turn_at_intersection(&mut self) {
        match self.next_turn {
            Turn::Left => {
                self.turn_left();
                self.next_turn = Turn::Straight;
            }
            Turn::Straight => {
                self.next_turn = Turn::Right;
            }
            Turn::Right => {
                self.turn_right();
                self.next_turn = Turn::Left;
            }
        }
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

fn get_carts(input: &Input) -> Vec<Cart> {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, cell)| match *cell {
                    '^' => Some(Cart::new(x, y, Direction::Up)),
                    'v' => Some(Cart::new(x, y, Direction::Down)),
                    '<' => Some(Cart::new(x, y, Direction::Left)),
                    '>' => Some(Cart::new(x, y, Direction::Right)),
                    _ => None,
                })
        })
        .collect()
}

fn simulate(input: &Input, bail_on_first_crash: bool) -> (usize, usize) {
    let mut carts: Vec<Cart> = get_carts(input);
    loop {
        // sort the carts for processing
        carts.sort();

        // every round, we need to keep track of the carts to kill
        let mut killed = vec![];

        for (idx, cart) in carts.clone().iter().enumerate() {
            let mut new_cart = *cart;
            // go in the current direction 1 step
            new_cart.next();

            // figure out if we need to turn, this could be better with modmath, but I'm lazy
            match (input[new_cart.y][new_cart.x], new_cart.direction) {
                ('/', Direction::Up) => new_cart.turn_right(),
                ('/', Direction::Down) => new_cart.turn_right(),
                ('/', Direction::Left) => new_cart.turn_left(),
                ('/', Direction::Right) => new_cart.turn_left(),
                ('\\', Direction::Up) => new_cart.turn_left(),
                ('\\', Direction::Down) => new_cart.turn_left(),
                ('\\', Direction::Left) => new_cart.turn_right(),
                ('\\', Direction::Right) => new_cart.turn_right(),
                ('+', _) => new_cart.turn_at_intersection(),
                _ => {}
            };

            for (other_idx, other) in carts.iter().enumerate() {
                if idx != other_idx && new_cart.x == other.x && new_cart.y == other.y {
                    if bail_on_first_crash {
                        return (new_cart.x, new_cart.y);
                    } else {
                        killed.push(idx);
                        killed.push(other_idx);
                        break;
                    }
                }
            }
            carts[idx] = new_cart;
        }

        // get rid of all the dead carts
        carts = carts
            .iter()
            .enumerate()
            .filter_map(|(idx, cart)| (!killed.contains(&idx)).then_some(*cart))
            .collect();

        if !bail_on_first_crash && carts.len() == 1 {
            let cart = carts.first().unwrap();
            return (cart.x, cart.y);
        }
    }
}

fn problem1(input: &Input) -> (usize, usize) {
    simulate(input, true)
}

fn problem2(input: &Input) -> (usize, usize) {
    simulate(input, false)
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, (7, 3))
    }

    #[test]
    fn second() {
        let input = include_str!("../test2.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, (6, 4))
    }
}
