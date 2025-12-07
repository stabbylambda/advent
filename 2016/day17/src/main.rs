use common::answer;
use std::{collections::BinaryHeap, vec};

use md5::{Digest, Md5};

fn main() {
    let input = "pxxbnzuo";

    let (min_path, max_len) = problem(input);
    answer!(min_path);
    answer!(max_len);
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    location: (u8, u8),
    steps: Vec<char>,
}

impl State {
    fn new() -> State {
        State {
            location: (0, 0),
            steps: Vec::new(),
        }
    }

    fn is_done(&self) -> bool {
        self.location == (3, 3)
    }

    fn get_open_doors(&self, password: &str) -> Vec<bool> {
        let b: Vec<u8> = self.steps.iter().map(|&c| c as u8).collect();

        let mut md5 = Md5::new();
        md5.update(password);
        md5.update(&b);
        let output = md5.finalize();

        // figure out which doors are open
        output[0..2]
            .iter()
            .flat_map(|&b| vec![(b >> 4) > 0xa, b & 0x0f > 0xa])
            .collect()
    }

    fn move_through_door(&self, direction: char) -> State {
        let (x, y) = self.location;
        let new_location = match direction {
            'U' => (x, y - 1),
            'D' => (x, y + 1),
            'L' => (x - 1, y),
            'R' => (x + 1, y),
            _ => unreachable!(),
        };

        let mut new_steps = self.steps.clone();
        new_steps.push(direction);

        State {
            location: new_location,
            steps: new_steps,
        }
    }

    fn available_directions(&self, password: &str) -> Vec<State> {
        let (x, y) = self.location;

        self.get_open_doors(password)
            .iter()
            .zip(DIRECTIONS)
            .filter_map(|(&open, direction)| {
                let can_move = open
                    && match direction {
                        'U' => y != 0,
                        'D' => y != 3,
                        'L' => x != 0,
                        'R' => x != 3,
                        _ => unreachable!(),
                    };

                can_move.then(|| self.move_through_door(direction))
            })
            .collect()
    }

    fn path_string(&self) -> String {
        self.steps.iter().collect()
    }

    fn len(&self) -> usize {
        self.steps.len()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.steps.len().cmp(&other.steps.len())
    }
}

const DIRECTIONS: [char; 4] = ['U', 'D', 'L', 'R'];

fn problem(input: &str) -> (String, usize) {
    let mut paths: Vec<State> = vec![];
    let mut priority_queue: BinaryHeap<State> = BinaryHeap::new();
    priority_queue.push(State::new());

    while let Some(state) = priority_queue.pop() {
        // if we're done, then check if this is better than we've ever done
        if state.is_done() {
            paths.push(state);
            continue;
        }

        // push all the new states into the queue
        priority_queue.extend(state.available_directions(input));
    }

    // get the best path string
    let min_path = paths.iter().min().unwrap().path_string();
    let max_path = paths.iter().max().unwrap().len();

    (min_path, max_path)
}

#[cfg(test)]
mod test {
    use crate::problem;
    #[test]
    fn first() {
        assert_eq!(problem("ihgpwlah"), ("DDRRRD".to_string(), 370usize));
        assert_eq!(problem("kglvqrro"), ("DDUDRLRRUDRD".to_string(), 492usize));
        assert_eq!(
            problem("ulqzkmiv"),
            ("DRURDRUDDLLDLUURRDULRLDUUDDDRR".to_string(), 830usize)
        );
    }
}
