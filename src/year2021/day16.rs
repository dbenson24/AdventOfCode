use crate::utils::*;

use nom::{
    IResult,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple,
    error::{Error, ParseError},
    multi::many0,
    InputIter,
    InputLength,
    Slice,
    lib::std::ops::{Shl, Shr, RangeFrom, AddAssign},
    ToUsize
};
use nom::bits::{bits, complete::take};

#[derive(Debug, Clone)]
pub struct Packet {
    version: usize,
    content: PacketContent
}

#[derive(Debug, Clone)]
pub enum PacketContent {
    Literal(i64),
    Operator(u8, Vec<Packet>)
}

impl Packet {
    pub fn count_versions(&self) -> usize {
        let child_version_sum = match &self.content {
            PacketContent::Operator(_, children) => children.iter().map(|x| x.count_versions()).sum(),
            _ => 0
        };
        return self.version + child_version_sum
    }
}

impl PacketContent {

    pub fn type_id(&self) -> u8 {
        match self {
            PacketContent::Literal(val) => 4,
            PacketContent::Operator(type_id, _) => *type_id
        }
    }

    pub fn eval(&self) -> i64 {
        match self {
            PacketContent::Literal(val) => *val,
            PacketContent::Operator(type_id, packets) => {
                let mut values = packets.iter().map(|x| x.content.eval());
                match *type_id {
                    0 => values.sum(),
                    1 => values.fold(1, |acc, x| acc * x),
                    2 => values.min().unwrap(),
                    3 => values.max().unwrap(),
                    5 => {
                        let first = values.next().unwrap();
                        let second = values.next().unwrap();
                        if first > second {
                            1
                        } else {
                            0
                        }
                    },
                    6 => {
                        let first = values.next().unwrap();
                        let second = values.next().unwrap();
                        if first < second {
                            1
                        } else {
                            0
                        }
                    },
                    7 => {
                        let first = values.next().unwrap();
                        let second = values.next().unwrap();
                        if first == second {
                            1
                        } else {
                            0
                        }
                    }
                    _x => panic!("unknown operator type {}", _x)
                }
            }
        }
    }
}

#[test]
pub fn base() {
    if let Ok(lines) = read_lines("./src/year2021/data/day16input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                let bytes = many0(hex_primary)(&contents).unwrap();
                let (i, (packet, packet_size)) = bits::<_, _, Error<(&[u8], usize)>, Error<&[u8]>, _>(parse_packet)(&bytes.1).unwrap();
                //dbg!(&packet);
                dbg!(packet.count_versions());
                dbg!(packet.content.type_id());
                dbg!(packet.content.eval());
            }
        }
    }
}


pub fn eval_packet(text: &str) -> i64 {
    let bytes = many0(hex_primary)(&text).unwrap();
    let (i, (packet, packet_size)) = bits::<_, _, Error<(&[u8], usize)>, Error<&[u8]>, _>(parse_packet)(&bytes.1).unwrap();
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
}



fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(2, 2, is_hex_digit),
        from_hex
    )(input)
}

fn packet_version(i: (&[u8], usize)) -> IResult<(&[u8], usize), usize> {
    take(3usize)(i)
}
fn packet_type(i: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    take(3usize)(i)
}
fn literal_value(i: (&[u8], usize)) -> IResult<(&[u8], usize), (u8, i64)> {
    tuple((take(1usize), take(4usize)))(i)
}
fn length_id(i: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    take(1usize)(i)
}

fn parse_packet(mut i: (&[u8], usize)) -> IResult<(&[u8], usize), (Packet, usize)> {
    let (i, version) = packet_version(i)?;
    let (mut i, id) = packet_type(i)?;
    let mut packet_size = 6usize;
    let packet = if id == 4 {
        let (j, (content, content_size)) = parse_literal(i, false)?;
        i = j;
        packet_size += content_size;
        Packet {
            version,
            content
        }
    } else {
        let (j, (content, content_size)) = parse_operator(i, id)?;
        i = j;
        packet_size += content_size;
        Packet {
            version,
            content
        }
    };

    Ok((i, (packet, packet_size)))
}

fn parse_literal(mut i: (&[u8], usize), trim_padding: bool) -> IResult<(&[u8], usize), (PacketContent, usize)> {
    let mut value = 0i64;
    let mut end = 1u8;
    let mut literal_size = 0usize;
    while end != 0 {
        let (j, (curr_end, curr_val)) = literal_value(i)?;
        i = j;
        end = curr_end;
        value = (value << 4) | curr_val;
        literal_size += 5;
    }
    if trim_padding {
        let padding = i.1 % 4;
        if padding > 0 {
            let (j, _): (_, u8) = take(4usize - padding)(i)?;
            i = j;
            literal_size += 4 - padding;
        }
    }
    Ok((i, (PacketContent::Literal(value), literal_size)))
}

fn parse_operator(i: (&[u8], usize), type_id: u8) -> IResult<(&[u8], usize), (PacketContent, usize)> {
    let (mut i, length_id) = length_id(i)?;
    let mut children = Vec::new();
    let mut packet_len = 1usize;
    if length_id == 0 {
        let (j, length): (_, usize) = take(15usize)(i)?;
        i = j;

        packet_len += length as usize;


        let mut curr_child_len = 0usize;
        while curr_child_len < length {
            if let Ok((j, (child, child_len))) = parse_packet(i) {
                curr_child_len += child_len;
                children.push(child);
                i = j;
            } else {
                break;
            }
        }
    } else {
        let (j, length): (_, usize) = take(11usize)(i)?;
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

fn collect_bit_slice(mut i: (&[u8], usize), len: usize) -> IResult<(&[u8], usize), Vec<u8>> {
    let taken = 0;
    let leftover = len % 8;
    let complete_len = len - leftover;
    let mut bytes = Vec::new();
    while taken < complete_len {
        let (j, b): (_, u8) = take(8usize)(i)?;
        i = j;
        bytes.push(b)
    }
    let (i, remaining): (_, u8) = take(leftover)(i)?;
    let remaining = remaining << (8 - leftover);
    bytes.push(remaining);
    Ok((i, bytes))
}
