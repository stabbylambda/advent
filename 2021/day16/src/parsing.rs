use std::num::ParseIntError;

use bitvec::prelude::*;

use crate::{Input, Packet, PacketType, Version};

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub type BVec = BitVec<u8, Msb0>;
pub type Slice = BitSlice<u8, Msb0>;

pub fn parse_operator(version: Version, type_id: u8, slice: &Slice) -> (Packet, &Slice) {
    let (length_id, rest) = slice.split_at(1);
    let (sub_packets, rest) = if length_id[0] {
        // read 11 bits to find the number of sub-packets
        let (head, rest) = rest.split_at(11);
        let sub_packet_count: u16 = head.load_be();

        parse_packets(rest, sub_packet_count as usize)
    } else {
        // read 15 to find the total length in bits
        let (head, rest) = rest.split_at(15);
        let total_length: u16 = head.load_be();

        let (sub_slice, rest) = rest.split_at(total_length as usize);
        let (packets, _) = parse_packets(sub_slice, usize::MAX);

        (packets, rest)
    };

    let t = match type_id {
        0 => PacketType::Sum,
        1 => PacketType::Product,
        2 => PacketType::Minimum,
        3 => PacketType::Maximum,
        5 => PacketType::GreaterThan,
        6 => PacketType::LessThan,
        7 => PacketType::EqualTo,
        _ => panic!(),
    };

    (Packet::Operator(version, t, sub_packets), rest)
}

pub fn parse_literal(version: Version, slice: &Slice) -> (Packet, &Slice) {
    let mut num: BVec = BitVec::new();
    let mut index = 0;
    loop {
        let continues = &slice[index];
        let slice = &slice[index + 1..index + 5];
        index += 5;

        // take the 4 bits from the end of this 5 bit slice
        num.extend_from_bitslice(slice);

        if !continues {
            break;
        }
    }
    let literal: u64 = num.load_be();

    let (_head, slice) = slice.split_at(index);

    (Packet::Literal(version, literal), slice)
}

const LITERAL_VERSION: u8 = 4;

pub fn parse(input: &str) -> Input {
    let b: BVec = decode_hex(input).unwrap().iter().collect();
    let slice = b.as_bitslice();
    let (packet, _rest) = parse_packet(&slice);
    packet
}

pub fn parse_packet(slice: &Slice) -> (Packet, &Slice) {
    let rest = slice;

    let (head, rest) = rest.split_at(3);
    let version: Version = head.load_be();

    let (head, rest) = rest.split_at(3);
    let type_id: u8 = head.load_be();

    match type_id {
        LITERAL_VERSION => parse_literal(version, rest),
        _ => parse_operator(version, type_id, rest),
    }
}

pub fn parse_packets(b: &Slice, max: usize) -> (Vec<Packet>, &Slice) {
    let mut slice = b;
    let mut packets: Vec<Packet> = Vec::new();

    while slice.any() && packets.len() < max {
        let (packet, rest) = parse_packet(&slice);
        packets.push(packet);
        slice = rest;
    }

    (packets, slice)
}
#[cfg(test)]
mod test {
    use crate::{parsing::parse, Packet, PacketType};

    #[test]
    fn parse_literal() {
        let input = parse("D2FE28");
        let expected = Packet::Literal(6, 2021);
        assert_eq!(input, expected);
    }

    #[test]
    fn parse_operator1() {
        let input = parse("38006F45291200");
        let expected = Packet::Operator(
            1,
            PacketType::LessThan,
            vec![Packet::Literal(6, 10), Packet::Literal(2, 20)],
        );
        assert_eq!(input, expected);
    }
    #[test]
    fn parse_operator2() {
        let input = parse("EE00D40C823060");
        let expected = Packet::Operator(
            7,
            PacketType::Maximum,
            vec![
                Packet::Literal(2, 1),
                Packet::Literal(4, 2),
                Packet::Literal(1, 3),
            ],
        );
        assert_eq!(input, expected);
    }
}
