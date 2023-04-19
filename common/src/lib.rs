pub mod dijkstra;
pub mod extensions;
pub mod heading;
pub mod map;
pub mod math;
pub mod nom;
pub mod orthogonal;
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
