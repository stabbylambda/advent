use common::nom::coord;

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

fn create_grid(serial_number: i32) -> Vec<Vec<i32>> {
    let mut cells = vec![vec![0; 300]; 300];
    for (y, row) in cells.iter_mut().enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            *cell = power_level(serial_number, (x + 1, y + 1));
        }
    }

    cells
}

fn max_subgrid(grid: &[&[i32]], size: usize) -> ((usize, usize), i32) {
    let mut max = i32::MIN;
    let mut coords = (0, 0);
    let mut windows = 0;

    for y in 0..300 - size {
        for x in 0..300 - size {
            let mut sum = 0;
            windows += 1;
            for dy in 0..size {
                for dx in 0..size {
                    sum += grid[y + dy][x + dx];
                }
            }

            if sum > max {
                max = sum;
                coords = (x + 1, y + 1);
            }
        }
    }
    dbg!(size, windows);

    (coords, max)
}

fn problem1(serial_number: i32) -> (usize, usize) {
    let grid = create_grid(serial_number);
    let slice: Vec<&[i32]> = grid.iter().map(|x| x.as_slice()).collect();
    let ((x, y), _power) = max_subgrid(&slice, 3);

    (x, y)
}

fn problem2(serial_number: i32) -> (usize, usize, usize) {
    let grid = create_grid(serial_number);
    let slice: Vec<&[i32]> = grid.iter().map(|x| x.as_slice()).collect();
    let mut max = i32::MIN;
    let mut coords = (0, 0, 0);

    for size in (1..300).rev() {
        let ((x, y), power) = max_subgrid(&slice, size);
        if power > max {
            max = power;
            coords = (x, y, size);
        }
    }

    coords
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

    #[ignore = "basically infinite runtime...gotta get better"]
    #[test]
    fn second() {
        let result = problem2(18);
        assert_eq!(result, (90, 269, 16));
        let result = problem2(42);
        assert_eq!(result, (232, 251, 12));
    }
}
