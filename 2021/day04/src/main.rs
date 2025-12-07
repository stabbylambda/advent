use ansi_term::Colour::Green;

use std::fmt::Display;

use common::{answer, read_input};

fn main() {
    let input = read_input!();
    let mut bingo = Bingo::new(input);

    answer!(bingo.play());
    answer!(bingo.determine_last_winner());
}

#[derive(Clone, Debug)]
struct Bingo {
    call_order: Vec<i32>,
    boards: Vec<Board>,
}
#[derive(Clone, Copy, Default, Debug)]
struct Cell {
    value: i32,
    marked: bool,
}

impl Cell {
    fn mark(&mut self) {
        self.marked = true
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = if self.marked {
            Green.paint(self.value.to_string()).to_string()
        } else {
            self.value.to_string()
        };

        write!(f, "{: >2} ", value)
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct Board {
    cells: [[Cell; 5]; 5],
    done: bool,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..5 {
            for col in 0..5 {
                let value = self.cells[row][col];
                write!(f, "{value}").expect("couldn't write a cell");
            }
            writeln!(f, " ").expect("couldn't write the newline");
        }

        f.write_str("")
    }
}

enum MarkResult {
    Win(i32),
    InProgress,
    AlreadyDone,
}

impl Board {
    fn new(input: &[&str]) -> Board {
        let mut cells = [[Cell::default(); 5]; 5];
        for row in 0..5 {
            let columns: Vec<i32> = input[row]
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            (0..5).for_each(|col| {
                let value = columns.get(col).unwrap().to_owned();
                let cell = Cell {
                    value,
                    marked: false,
                };
                cells[row][col] = cell;
            });
        }
        Board { cells, done: false }
    }

    fn is_a_winner(&self) -> bool {
        for idx in 0..5 {
            // check the row
            let row = self.cells[idx];
            let row_winner = row.iter().all(|x| x.marked);

            // gather the column and check it
            let col_winner = (0..5).map(|y| self.cells[y][idx]).all(|c| c.marked);

            if row_winner || col_winner {
                return true;
            }
        }

        false
    }

    fn play(&mut self, number: i32) -> MarkResult {
        // don't mark more if it's already won
        if self.done {
            return MarkResult::AlreadyDone;
        }

        // mark all of the current number as scored
        for x in 0..5 {
            for y in 0..5 {
                if self.cells[x][y].value == number {
                    self.cells[x][y].mark();
                }
            }
        }

        // check if we're a winner and mark done
        if self.is_a_winner() {
            self.done = true;
            MarkResult::Win(self.score(number))
        } else {
            MarkResult::InProgress
        }
    }

    fn score(&self, winning_number: i32) -> i32 {
        // add all the unmarked cells
        let sum = self.cells.iter().fold(0, |acc, x| {
            x.iter().filter(|c| !c.marked).fold(acc, |a, b| a + b.value)
        });
        sum * winning_number
    }
}

impl Bingo {
    fn new(input: &str) -> Bingo {
        let input: Vec<&str> = input.lines().collect();
        let call_order: Vec<i32> = input
            .first()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let boards = input[1..].chunks(6).map(|b| Board::new(&b[1..])).collect();

        Bingo { call_order, boards }
    }

    fn play(&mut self) -> i32 {
        for d in &self.call_order {
            for b in &mut self.boards[..] {
                match b.play(*d) {
                    MarkResult::Win(score) => {
                        println!("winner on {d}!\n{b}");
                        return score;
                    }
                    _ => continue,
                };
            }
        }
        panic!("No winners!")
    }

    fn determine_last_winner(&mut self) -> i32 {
        for d in &self.call_order {
            let loser_count;
            {
                let losers = self.boards.iter().filter(|b| !b.is_a_winner());
                loser_count = losers.count();
            }

            for b in &mut self.boards[..] {
                match b.play(*d) {
                    MarkResult::Win(score) => {
                        if loser_count == 1 {
                            return score;
                        }
                    }
                    _ => continue,
                }
            }
        }
        panic!("No winners!")
    }
}

#[test]
fn first() {
    let input = include_str!("../test.txt");
    let mut data = Bingo::new(input);
    let result = data.play();
    assert_eq!(4512, result);
}

#[test]
fn second() {
    let input = include_str!("../test.txt");
    let mut data = Bingo::new(input);
    let result = data.determine_last_winner();
    assert_eq!(1924, result);
}

#[test]
fn winner() {
    let b = Board::new(&[
        "0 1 2 3 4",
        "0 1 2 3 4",
        "0 1 2 3 4",
        "0 1 2 3 4",
        "0 1 2 3 4",
    ]);

    let mut col = b;
    for x in 0..5 {
        col.cells[0][x].mark();
    }
    assert!(col.is_a_winner());

    let mut row = b;
    for x in 0..5 {
        row.cells[x][2].mark();
    }
    assert!(row.is_a_winner());
}

#[test]
fn score() {
    let b = Board::new(&[
        "14 21 17 24  4",
        "10 16 15  9 19",
        "18  8 23 26 20",
        "22 11 13  6  5",
        " 2  0 12  3  7",
    ]);

    let draws = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24];
    let mut bingo = Bingo {
        boards: vec![b],
        call_order: draws,
    };

    let result = bingo.play();

    assert_eq!(4512, result);
}
