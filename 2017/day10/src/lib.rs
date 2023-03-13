use std::collections::VecDeque;

pub fn hash_str(input: &str) -> u128 {
    let mut input: Vec<u8> = input.bytes().collect();
    input.extend_from_slice(&[17, 31, 73, 47, 23]);

    let sparse = hash(input, 256, 64);
    let dense: Vec<u8> = sparse
        .chunks(16)
        .map(|x| x.iter().fold(0, |a, b| a ^ b))
        .collect();

    let dense: [u8; 16] = dense.try_into().unwrap();

    u128::from_be_bytes(dense)
}

pub fn hash(input: Vec<u8>, max: usize, rounds: usize) -> Vec<u8> {
    let mut rope: VecDeque<usize> = (0..max).collect();
    let mut current = 0;
    let mut skip = 0;

    for _round in 0..rounds {
        for length in &input {
            let length = *length as usize;
            /* Rotate to make the current at 0, reverse up to the length,
            then rotate back so we're in the right spot again */
            rope.rotate_left(current);
            rope.make_contiguous()[0..length].reverse();
            rope.rotate_right(current);

            // Move the current position forward by that length plus the skip size.
            current = (current + length + skip) % max;
            skip += 1;
        }
    }

    rope.iter().cloned().map(|x| x as u8).collect::<Vec<u8>>()
}

pub trait SparseHash {
    fn check(&self) -> u8;
    fn to_dense(&self) -> String;
}

impl SparseHash for Vec<u8> {
    fn check(&self) -> u8 {
        self[0] * self[1]
    }

    fn to_dense(&self) -> String {
        let dense: String = self
            .chunks(16)
            .map(|x| x.iter().fold(0, |a, b| a ^ b))
            .map(|c| format!("{:02x}", c))
            .collect();

        dense
    }
}
