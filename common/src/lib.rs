pub mod dijkstra;
pub mod extensions;
pub mod grid;
pub mod heading;
pub mod math;
pub mod nom;
pub mod program;
pub mod union_find;

pub fn to_number(value: &[u32]) -> u32 {
    value.iter().fold(0, |acc, x| (acc * 10) + x)
}

pub fn digits(input: usize) -> Vec<u8> {
    let mut input = input;
    let mut v = vec![];

    if input == 0 {
        return vec![0];
    }

    while input != 0 {
        let digit = input % 10;
        input /= 10;

        v.push(digit as u8);
    }

    v.reverse();
    v
}

pub fn transpose<T: Clone + Copy>(input: &[Vec<T>]) -> Vec<Vec<T>> {
    let width = input[0].len();
    // transpose the nested vec so we can examine each char index
    let mut i_t: Vec<Vec<T>> = vec![vec![]; width];
    (0..width).for_each(|x| {
        (0..input.len()).for_each(|y| i_t[x].push(input[y][x]));
    });

    i_t
}
