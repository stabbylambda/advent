use common::answer;

fn main() {
    let row = 2981;
    let column = 3075;
    answer!(problem1(row, column));
}

fn triangle(row: u64, col: u64) -> u64 {
    let triangle = (row + col - 1) * (row + col) / 2;
    triangle - row + 1
}

fn problem1(row: u64, col: u64) -> u64 {
    let idx = triangle(row, col);
    let code = 20_151_125;

    (1..idx).fold(code, |acc, _x| (acc * 252_533) % 33_554_393)
}

#[cfg(test)]
mod test {

    use crate::problem1;
    #[test]
    fn first() {
        let result = problem1(2981, 3075);
        assert_eq!(result, 9_132_360)
    }
}
