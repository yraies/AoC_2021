use itertools::{FoldWhile, Itertools};

//use aoc_runner_derive::{aoc, aoc_generator};

#[main]
#[allow(dead_code)]
pub fn main() {
    let _input = std::fs::read_to_string("input/day16.txt").expect("Could not find day 16 data!");
    //let _testinput = "D2FE28"; // Literal test
    let _testinput = "38006F45291200"; // Total-Length Subpackets test
    let _testinput = "EE00D40C823060"; // Nr of subpackets test
    let _testinput = "A0016C880162017C3686B18A3D4780"; // Total version sum test
    let parsed_data = parse_data(&_input);
    println!("Part 1: {}", part1(&parsed_data));
    println!("Part 2: {}", part2(&parsed_data));
}

/// Standard Packet Header: VVVTTT
/// V: 3bit Version
/// T: 3bit Type ID
///
/// Literal Packets: VVV110[1NNNN]*0NNNN0*
/// N: 5bit blocks of the literal
///
/// Operator Packets: VVVTTTLD0*
/// L: 1bit Length Type ID
///     - 0: total length of subpackets D=
///     - 1: number of subpackets
#[derive(Debug)]
pub enum Packet {
    Literal {
        version: u8,
        data: u64,
    },
    Operator {
        version: u8,
        op_type: u8,
        data: Vec<Packet>,
    },
}

pub struct PacketBuilder;

impl PacketBuilder {
    fn fold_u8_chunk(acc: u8, next: &bool) -> u8 {
        (acc << 1) + if *next { 1 } else { 0 }
    }

    fn fold_u16_chunk(acc: u16, next: &bool) -> u16 {
        (acc << 1) + if *next { 1 } else { 0 }
    }

    fn parse_number_literal(bits: &[bool]) -> (u64, usize) {
        let mut consumed_bits = 0;
        (
            bits.chunks(5)
                .fold_while(0u64, |acc, chunk| {
                    consumed_bits += 5;
                    if chunk[0] {
                        FoldWhile::Continue(
                            (acc << chunk[1..].len())
                                + chunk[1..].iter().fold(0, PacketBuilder::fold_u8_chunk) as u64,
                        )
                    } else {
                        FoldWhile::Done(
                            (acc << chunk[1..].len())
                                + chunk[1..].iter().fold(0, PacketBuilder::fold_u8_chunk) as u64,
                        )
                    }
                })
                .into_inner(),
            consumed_bits,
        )
    }

    fn parse_package(bits: &[bool], skip_end_zeros: bool) -> (Packet, usize) {
        //println!(
        //    "Parsing: {}",
        //    bits.iter().map(|b| if *b { "1" } else { "0" }).join("")
        //);

        let version = bits[0..3].iter().fold(0u8, PacketBuilder::fold_u8_chunk);
        let op_type = bits[3..6].iter().fold(0u8, PacketBuilder::fold_u8_chunk);
        let mut consumed_bits = 6;

        let packet = if op_type == 4u8 {
            //println!("Found literal number!");
            let (data, data_len) = PacketBuilder::parse_number_literal(&bits[consumed_bits..]);
            consumed_bits += data_len;
            Packet::Literal { version, data }
        } else {
            let mut data = vec![];

            if !bits[consumed_bits] {
                // ie. total length
                consumed_bits += 1;
                let total_length = bits[consumed_bits..consumed_bits + 15]
                    .iter()
                    .fold(0u16, PacketBuilder::fold_u16_chunk)
                    as usize;
                //println!(
                //    "Found operator with {} bits of sub-packets by {:?}!",
                //    total_length,
                //    bits[consumed_bits..consumed_bits + 15]
                //        .iter()
                //        .map(|b| if *b { "1" } else { "0" })
                //        .join("")
                //);
                consumed_bits += 15;
                let mut curr_length = 0usize;
                while curr_length < total_length {
                    let (packet, size) =
                        PacketBuilder::parse_package(&bits[consumed_bits..], false);
                    curr_length += size;
                    consumed_bits += size;
                    data.push(packet);
                }
            } else {
                consumed_bits += 1;
                let packet_count = bits[consumed_bits..consumed_bits + 11]
                    .iter()
                    .fold(0u16, PacketBuilder::fold_u16_chunk)
                    as usize;
                //println!("Found operator with {} sub-packets!", packet_count);
                consumed_bits += 11;
                for _ in 0..packet_count {
                    let (packet, size) =
                        PacketBuilder::parse_package(&bits[consumed_bits..], false);
                    consumed_bits += size;
                    data.push(packet);
                }
            }

            Packet::Operator {
                version,
                op_type,
                data,
            }
        };

        //println!(
        //    "{} resulted in {:?}",
        //    bits.iter().map(|b| if *b { "1" } else { "0" }).join(""),
        //    packet
        //);

        (
            packet,
            consumed_bits + if skip_end_zeros { consumed_bits % 4 } else { 0 },
        )
    }
}

//#[aoc_generator(day)]
pub fn parse_data(input: &str) -> Vec<Packet> {
    let mut packets = vec![];
    let bits = input
        .trim()
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u8)
        .flat_map(|nr| {
            (0..4)
                .into_iter()
                .rev()
                .map(|shift| ((nr >> shift) & 0b1) != 0)
                .collect_vec()
                .into_iter()
        })
        .collect_vec();

    let mut start_idx = 0;

    while start_idx < bits.len() && !bits[start_idx..].iter().all(|b| !b) {
        let (packet, packet_size) = PacketBuilder::parse_package(&bits[start_idx..], false);
        packets.push(packet);
        start_idx += packet_size;
    }

    packets
}

impl Packet {
    #[allow(dead_code)]
    fn get_version(&self) -> u8 {
        match self {
            Packet::Literal { version, .. } | Packet::Operator { version, .. } => *version,
        }
    }

    fn get_total_version(&self) -> usize {
        match self {
            Packet::Literal { version, .. } => *version as usize,
            Packet::Operator { version, data, .. } => {
                (*version as usize)
                    + (data.iter().map(|p| p.get_total_version()).sum::<usize>() as usize)
            }
        }
    }

    #[allow(dead_code)]
    fn get_type(&self) -> u8 {
        match self {
            Packet::Literal { .. } => 4u8,
            Packet::Operator { op_type, .. } => *op_type,
        }
    }

    fn eval(&self) -> usize {
        match self {
            Packet::Literal { data, .. } => *data as usize,
            Packet::Operator { op_type, data, .. } => match op_type {
                0 => data.iter().map(|p| p.eval()).sum(),
                1 => data
                    .iter()
                    .map(|p| p.eval())
                    .fold(1, |acc, next| acc * next),
                2 => data.iter().map(|p| p.eval()).min().unwrap(),
                3 => data.iter().map(|p| p.eval()).max().unwrap(),
                5 => {
                    if data[0].eval() > data[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if data[0].eval() < data[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if data[0].eval() == data[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                _ => usize::MAX,
            },
        }
    }
}

//#[aoc(day, part1)]
pub fn part1(input: &Vec<Packet>) -> usize {
    //println!("{:?}", input);
    input
        .iter()
        .map(|pckg| pckg.get_total_version() as usize)
        .sum()
}

//#[aoc(day, part2)]
pub fn part2(input: &Vec<Packet>) -> usize {
    assert!(input.len() == 1);
    input[0].eval()
}
