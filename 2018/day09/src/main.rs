use std::collections::VecDeque;

use common::answer;

fn main() {
    let input = (446, 71522);

    answer!(problem1(input));
    answer!(problem2(input));
}

type Input = (usize, u32);

fn problem1((players, last_marble): Input) -> u32 {
    let mut circle = VecDeque::new();
    circle.push_back(0);
    let mut scores = vec![0; players];

    for marble in 1..=last_marble {
        let player = (marble - 1) as usize % players;

        if marble % 23 == 0 {
            circle.rotate_right(8);
            let score = marble + circle.pop_front().unwrap();
            scores[player] += score;
            circle.rotate_left(1);
        } else {
            circle.rotate_left(1);
            circle.push_back(marble);
        }
    }

    *scores.iter().max().unwrap()
}

fn problem2((players, last_marble): Input) -> u32 {
    problem1((players, last_marble * 100))
}

#[cfg(test)]
mod test {
    use crate::problem1;
    #[test]
    fn first() {
        let examples = [
            ((9, 25), 32),
            ((10, 1618), 8317),
            ((13, 7999), 146373),
            ((17, 1104), 2764),
            ((21, 6111), 54718),
            ((30, 5807), 37305),
        ];
        for (input, expected) in examples {
            let result = problem1(input);
            assert_eq!(result, expected);
        }
    }
}
