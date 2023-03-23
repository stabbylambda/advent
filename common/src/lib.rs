pub mod dijkstra;
pub mod extensions;
pub mod map;
pub mod nom;
pub mod orthogonal;
pub mod program;
pub mod union_find;

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
