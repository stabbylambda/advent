use std::{collections::BTreeMap, mem::swap};

use nom::{
    bytes::complete::tag,
    character::complete::{newline, u64},
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = (Player, Player);

fn parse(input: &str) -> Input {
    let player = |s| {
        map(
            separated_pair(
                preceded(tag("Player "), u64),
                tag(" starting position: "),
                u64,
            ),
            |(_id, position)| Player::new(position),
        )(s)
    };
    let result: IResult<&str, Input> = separated_pair(player, newline, player)(input);

    result.unwrap().1
}

struct DeterministicDie {
    value: u64,
    rolls: u64,
}

impl DeterministicDie {
    fn new(value: u64) -> Self {
        Self { value, rolls: 0 }
    }

    fn roll(&mut self) -> u64 {
        let move_total = (0..3).fold(0, |acc, _x| {
            self.value = match (self.value + 1) % 100 {
                0 => 100,
                x => x,
            };
            acc + self.value
        });
        self.rolls += 3;
        move_total
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Player {
    position: u64,
    score: u64,
}

impl Player {
    fn new(position: u64) -> Self {
        Self { position, score: 0 }
    }

    fn turn(&mut self, move_total: u64) -> u64 {
        self.position = (self.position + move_total) % 10;
        self.score += match self.position {
            0 => 10,
            x => x,
        };

        self.score
    }
}

fn problem1(input: &Input) -> u64 {
    let (mut p1, mut p2) = *input;
    let mut die = DeterministicDie::new(0);

    loop {
        let roll = die.roll();
        if p1.turn(roll) >= 1000 {
            break;
        }

        // swap players every turn
        swap(&mut p1, &mut p2);
    }

    let min_score = p1.score.min(p2.score);
    die.rolls * min_score
}

// these are all possible outcomes from the Dirac Dice
const ROLLS: [(u64, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
type Cache = BTreeMap<(Player, Player), (u64, u64)>;

fn dirac_game(cache: &mut Cache, p1: Player, p2: Player) -> (u64, u64) {
    // have we already seen this game?
    if let Some(&score) = cache.get(&(p1, p2)) {
        score
    } else {
        let (score1, score2) = ROLLS
            .into_iter()
            .fold((0, 0), |(score1, score2), (die, times)| {
                let mut p1 = p1;
                p1.turn(die);

                let (wins1, wins2) = if p1.score >= 21 {
                    (1, 0)
                } else {
                    dirac_game(cache, p2, p1)
                };

                (score1 + wins1 * times, score2 + wins2 * times)
            });

        // gotta return in reverse order because the roles are reversed every other recursion
        cache.insert((p1, p2), (score2, score1));
        (score2, score1)
    }
}

fn problem2(input: &Input) -> u64 {
    let (p1, p2) = *input;
    let (wins1, wins2) = dirac_game(&mut BTreeMap::new(), p1, p2);

    wins1.max(wins2)
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 739785)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 444_356_092_776_315u64)
    }
}
