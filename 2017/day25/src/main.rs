use common::answer;
use std::collections::VecDeque;

fn main() {
    answer!(problem1());
}

#[derive(PartialEq, Eq)]
enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}

fn problem1() -> usize {
    /* This wasn't worth parsing. I just decided to code up the states manually. */
    let steps = 12_994_925;

    let mut state = State::A;
    /* trial and error got me to this number for the tape. I started with an equivalent number of steps,
    got the right answer, then backed it down until it started failing. The actual number of tape cells
    used is 5692...but that's neither here nor there. */
    let tape_size = 6000;
    let mut tape: VecDeque<bool> = VecDeque::from_iter((0..tape_size).map(|_| false));

    for _step in 0..steps {
        let current = tape.front_mut().unwrap();
        state = match (state, *current) {
            (State::A, false) => {
                *current = true;
                tape.rotate_right(1);
                State::B
            }
            (State::A, true) => {
                *current = false;
                tape.rotate_left(1);
                State::F
            }
            (State::B, false) => {
                *current = false;
                tape.rotate_right(1);
                State::C
            }
            (State::B, true) => {
                *current = false;
                tape.rotate_right(1);
                State::D
            }
            (State::C, false) => {
                *current = true;
                tape.rotate_left(1);
                State::D
            }
            (State::C, true) => {
                *current = true;
                tape.rotate_right(1);
                State::E
            }
            (State::D, false) => {
                *current = false;
                tape.rotate_left(1);
                State::E
            }
            (State::D, true) => {
                *current = false;
                tape.rotate_left(1);
                State::D
            }
            (State::E, false) => {
                *current = false;
                tape.rotate_right(1);
                State::A
            }
            (State::E, true) => {
                *current = true;
                tape.rotate_right(1);
                State::C
            }
            (State::F, false) => {
                *current = true;
                tape.rotate_left(1);
                State::A
            }
            (State::F, true) => {
                *current = true;
                tape.rotate_right(1);
                State::A
            }
        }
    }

    tape.iter().filter(|x| **x).count()
}

#[cfg(test)]
mod test {
    use crate::problem1;
    #[test]
    fn first() {
        let result = problem1();
        assert_eq!(result, 2846)
    }
}
