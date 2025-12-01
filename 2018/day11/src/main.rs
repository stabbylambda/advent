use std::ops::Range;

fn main() {
    let input = 8979;

    let (x, y) = problem1(input);
    println!("problem 1 answer: {x},{y}");

    let (x, y, size) = problem2(input);
    println!("problem 2 answer: {x},{y},{size}");
}

fn power_level(serial_number: i32, (x, y): (usize, usize)) -> i32 {
    let x = x as i32;
    let y = y as i32;

    let rack_id = x + 10;
    let power = rack_id * y;
    let power = power + serial_number;
    let power = power * rack_id;
    let hundreds = (power / 100) % 10;

    hundreds - 5
}

// This is a https://en.wikipedia.org/wiki/Summed-area_table
fn create_summed_area_table(serial_number: i32) -> Vec<Vec<i32>> {
    let mut cells = vec![vec![0; 300]; 300];
    for y in 0..300 {
        for x in 0..300 {
            let p = power_level(serial_number, (x + 1, y + 1));
            let prev_y = if y > 0 { cells[y - 1][x] } else { Default::default() };
            let prev_x = if x > 0 { cells[y][x - 1] } else { Default::default() };
            let prev_xy = if x > 0 && y > 0 { cells[y - 1][x - 1] } else { Default::default() };

            cells[y][x] = p + prev_y + prev_x - prev_xy;
        }
    }

    cells
}

fn find_best_subgrid(serial_number: i32, size: Range<usize>) -> (usize, usize, usize) {
    let grid = create_summed_area_table(serial_number);

    let mut max = i32::MIN;
    let mut coords = (0, 0, 0);

    // this is a sum of intensities over a rectangular area on the grid
    for s in size {
        for y in s..300 {
            for x in s..300 {
                let d = grid[y][x];
                let b = grid[y - s][x];
                let c = grid[y][x - s];
                let a = grid[y - s][x - s];

                let total = d - b - c + a;

                if total > max {
                    max = total;
                    // need to adjust for stupid off-by-one because the coords are 1 based
                    // and we're subtracting sizes
                    coords = (x - s + 2, y - s + 2, s);
                }
            }
        }
    }

    coords
}

fn problem1(serial_number: i32) -> (usize, usize) {
    let (x, y, _) = find_best_subgrid(serial_number, 3..4);
    (x, y)
}

fn problem2(serial_number: i32) -> (usize, usize, usize) {
    find_best_subgrid(serial_number, 1..300)
}

#[cfg(test)]
mod test {
    use crate::{power_level, problem1, problem2};
    #[test]
    fn power() {
        let cases = [
            (8, (3, 5), 4),
            (57, (122, 79), -5),
            (39, (217, 196), 0),
            (71, (101, 153), 4),
        ];
        for (serial_number, coords, expected) in cases {
            let result = power_level(serial_number, coords);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn first() {
        let result = problem1(18);
        assert_eq!(result, (33, 45));
        let result = problem1(42);
        assert_eq!(result, (21, 61));
    }

    #[test]
    fn second() {
        let result = problem2(18);
        assert_eq!(result, (90, 269, 16));
        let result = problem2(42);
        assert_eq!(result, (232, 251, 12));
    }
}
