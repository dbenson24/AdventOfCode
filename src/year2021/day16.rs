use std::str::FromStr;

use crate::utils::*;
use nom::bits::{bits, complete::take};
use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    error::{Error, ParseError},
    lib::std::ops::{AddAssign, RangeFrom, Shl, Shr},
    multi::many0,
    sequence::tuple,
    IResult, InputIter, InputLength, Slice, ToUsize,
};

#[derive(Debug, Clone)]
pub struct Packet {
    version: usize,
    content: PacketContent,
}

#[derive(Debug, Clone)]
pub enum PacketContent {
    Literal(u64),
    Operator(u8, Vec<Packet>),
}

impl Packet {
    pub fn count_versions(&self) -> usize {
        let child_version_sum = match &self.content {
            PacketContent::Operator(_, children) => {
                children.iter().map(|x| x.count_versions()).sum()
            }
            _ => 0,
        };
        return self.version + child_version_sum;
    }
}

#[derive(Debug)]
pub struct PacketParseError();
impl FromStr for Packet {
    type Err = PacketParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = many0(hex_primary)(s).unwrap();
        if bytes.0.len() > 0 {
            let byte = u8::from_str_radix(&bytes.0, 16).unwrap();
            bytes.1.push(byte << 4);
        }
        let (i, (packet, packet_size)) =
            bits::<_, _, Error<(&[u8], usize)>, Error<&[u8]>, _>(parse_packet)(&bytes.1).unwrap();

        Ok(packet)
    }
}

impl PacketContent {
    pub fn type_id(&self) -> u8 {
        match self {
            PacketContent::Literal(_) => 4,
            PacketContent::Operator(type_id, _) => *type_id,
        }
    }

    pub fn eval(&self) -> u64 {
        match self {
            PacketContent::Literal(val) => *val,
            PacketContent::Operator(type_id, packets) => {
                let mut values: Vec<_> = packets.iter().map(|x| x.content.eval()).collect();
                let mut values = values.into_iter();
                match *type_id {
                    0 => values.sum(),
                    1 => values.fold(1, |acc, x| acc * x),
                    2 => values.min().unwrap(),
                    3 => values.max().unwrap(),
                    5 => (values.next().unwrap() > values.next().unwrap()) as u64,
                    6 => (values.next().unwrap() < values.next().unwrap()) as u64,
                    7 => (values.next().unwrap() == values.next().unwrap()) as u64,
                    _x => panic!("unknown operator type {}", _x),
                }
            }
        }
    }
}

pub fn day_16() {
    if let Ok(lines) = read_lines("./src/year2021/data/day16input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                dbg!(eval_packet(&contents));
            }
        }
    }
}

pub fn eval_packet(text: &str) -> u64 {
    let packet: Packet = text.parse().unwrap();
    packet.content.eval()
}

#[test]
pub fn test_1() {
    assert_eq!(eval_packet("C200B40A82"), 3);
    assert_eq!(eval_packet("04005AC33890"), 54);
    assert_eq!(eval_packet("880086C3E88112"), 7);
    assert_eq!(eval_packet("CE00C43D881120"), 9);
    assert_eq!(eval_packet("D8005AC2A8F0"), 1);
    assert_eq!(eval_packet("F600BC2D8F"), 0);
    assert_eq!(eval_packet("9C005AC2F8F0"), 0);
    assert_eq!(eval_packet("9C0141080250320F1802104A08"), 1);
    assert_eq!(eval_packet("3232D42BF9400"), 5000000000);
    assert_eq!(
        eval_packet("3600888023024c01150044c0118330a440118330e44011833085c0118522008c29870"),
        1
    );
}

fn parse_packet(mut i: (&[u8], usize)) -> IResult<(&[u8], usize), (Packet, usize)> {
    let (i, version) = packet_version(i)?;
    let (i, id) = packet_type(i)?;
    let mut packet_size = 6usize;

    let (i, (content, content_size)) = if id == 4 {
        parse_literal(i)?
    } else {
        parse_operator(i, id)?
    };
    packet_size += content_size;
    let packet = Packet { version, content };

    Ok((i, (packet, packet_size)))
}

fn parse_literal(mut i: (&[u8], usize)) -> IResult<(&[u8], usize), (PacketContent, usize)> {
    let mut value = 0u64;
    let mut end = 1u8;
    let mut literal_size = 0usize;
    while end != 0 {
        let (j, (curr_end, curr_val)) = literal_value(i)?;
        i = j;
        end = curr_end;
        value = (value << 4) | curr_val;
        literal_size += 5;
    }

    Ok((i, (PacketContent::Literal(value), literal_size)))
}

fn parse_operator(
    i: (&[u8], usize),
    type_id: u8,
) -> IResult<(&[u8], usize), (PacketContent, usize)> {
    let (mut i, length_id) = length_id(i)?;
    let mut children = Vec::new();
    let mut packet_len = 1usize;
    if length_id == 0 {
        let (child_start, length) = take_u64(i, 15)?;
        i = child_start;
        packet_len += length as usize;

        let mut curr = i;
        let needed_bytes = ((curr.1 + length as usize) as f64 / 8.).ceil() as usize;
        curr.0 = &curr.0[0..needed_bytes];
        let mut curr_child_len = 0usize;
        while curr_child_len < length as usize {
            if let Ok((j, (child, child_len))) = parse_packet(curr) {
                curr_child_len += child_len;
                children.push(child);
                curr = j;
            } else {
                break;
            }
        }

        i = skip_bits(i, length as usize);
    } else {
        let (j, length) = take_u64(i, 11)?;
        i = j;
        for _ in 0..length {
            if let Ok((j, (child, child_len))) = parse_packet(i) {
                i = j;
                packet_len += child_len;
                children.push(child);
            } else {
                break;
            }
        }
    }

    Ok((i, (PacketContent::Operator(type_id, children), packet_len)))
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn packet_version(i: (&[u8], usize)) -> IResult<(&[u8], usize), usize> {
    take(3usize)(i)
}
fn packet_type(i: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    take(3usize)(i)
}
fn literal_value(i: (&[u8], usize)) -> IResult<(&[u8], usize), (u8, u64)> {
    tuple((take(1usize), take(4usize)))(i)
}
fn length_id(i: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    take(1usize)(i)
}

fn take_u64(i: (&[u8], usize), len: usize) -> IResult<(&[u8], usize), u64> {
    take(len)(i)
}

fn skip_bits(i: (&[u8], usize), len: usize) -> (&[u8], usize) {
    let dest_byte = (i.1 + len) / 8;
    let offset = (i.1 + len) % 8;
    if dest_byte < i.0.len() {
        (&i.0[dest_byte..], offset)
    } else {
        (&i.0[i.0.len()..], 0)
    }
}