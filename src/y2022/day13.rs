use std::{
    cmp::Ordering,
    fmt::{Display, Write},
    str::FromStr,
};

use itertools::Itertools;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let packet_pairs = input
        .trim()
        .split("\n\n")
        .map(|lines| {
            let (a, b) = lines.split_once('\n').unwrap();
            (a.parse::<Packet>().unwrap(), b.parse::<Packet>().unwrap())
        })
        .collect::<Vec<_>>();

    let part1 = packet_pairs
        .iter()
        .enumerate()
        .filter(|(_, (a, b))| a < b)
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    let mut packets = packet_pairs
        .into_iter()
        .flat_map(|(a, b)| [a, b].into_iter())
        .collect::<Vec<_>>();
    let dividers = [2, 6]
        .into_iter()
        .map(|v| Packet {
            values: vec![PacketValue::Packet(Packet {
                values: vec![PacketValue::Number(v)],
            })],
        })
        .collect::<Vec<_>>();
    dividers.iter().cloned().for_each(|d| packets.push(d));
    packets.sort();
    let part2 = dividers
        .iter()
        .map(|d| {
            packets
                .iter()
                .find_position(|&packet| packet == d)
                .unwrap()
                .0
                + 1
        })
        .product::<usize>();

    (part1, part2)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Packet {
    values: Vec<PacketValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketValue {
    Packet(Packet),
    Number(i64),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            return Ordering::Equal;
        }
        for items in self.values.iter().zip(other.values.iter()) {
            match items {
                (PacketValue::Number(a), PacketValue::Number(b)) => {
                    if a < b {
                        return Ordering::Less;
                    }
                    if a > b {
                        return Ordering::Greater;
                    }
                }
                (PacketValue::Packet(a), PacketValue::Packet(b)) => {
                    let result = a.partial_cmp(b);
                    if let Some(result) = result {
                        return result;
                    }
                }
                (PacketValue::Number(a), PacketValue::Packet(b)) => {
                    let tmp_packet = Packet {
                        values: vec![PacketValue::Number(*a)],
                    };
                    let result = tmp_packet.partial_cmp(b);
                    if let Some(result) = result {
                        return result;
                    }
                }
                (PacketValue::Packet(a), PacketValue::Number(b)) => {
                    let tmp_packet = Packet {
                        values: vec![PacketValue::Number(*b)],
                    };
                    let result = a.partial_cmp(&tmp_packet);
                    if let Some(result) = result {
                        return result;
                    }
                }
            }
        }

        if self.values.len() < other.values.len() {
            return Ordering::Less;
        }
        if self.values.len() > other.values.len() {
            return Ordering::Greater;
        }

        Ordering::Equal
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars().peekable();
        let mut packet_stack = Vec::new();
        while let Some(c) = chars.next() {
            match c {
                ',' => (),
                '[' => {
                    packet_stack.push(Packet { values: Vec::new() });
                }
                ']' => {
                    let packet = packet_stack.pop().unwrap();
                    match packet_stack.last_mut() {
                        Some(last) => last.values.push(PacketValue::Packet(packet)),
                        None => {
                            assert!(chars.peek().is_none());
                            assert_eq!(s, packet.to_string());
                            return Ok(packet);
                        }
                    }
                }
                '0'..='9' => {
                    let mut digits = vec![c];
                    while matches!(chars.peek(), Some('0'..='9')) {
                        digits.push(chars.next().unwrap());
                    }
                    let number = digits.into_iter().collect::<String>().parse().unwrap();
                    packet_stack
                        .last_mut()
                        .unwrap()
                        .values
                        .push(PacketValue::Number(number));
                }
                _ => unreachable!(),
            }
        }

        unreachable!()
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        for (i, value) in self.values.iter().enumerate() {
            if i != 0 {
                f.write_char(',')?;
            }
            match value {
                PacketValue::Number(v) => f.write_str(&v.to_string())?,
                PacketValue::Packet(packet) => packet.fmt(f)?,
            }
        }
        f.write_str("]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        let result = solve(input);

        assert_eq!(result.0.to_string(), 13.to_string());
        assert_eq!(result.1.to_string(), 140.to_string());
    }
}
