pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let data = input
        .trim()
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u8)
        .collect::<Vec<_>>();
    let mut reader = PacketReader::from_bytes(&data);
    let root_packet = reader.read_packet();

    let mut total = 0_u64;
    let mut queue = vec![&root_packet];
    while !queue.is_empty() {
        let curr = queue.pop().unwrap();
        match curr {
            Packet::Literal { version, .. } => total += *version as u64,
            Packet::Sum {
                version,
                sub_packets,
            }
            | Packet::Product {
                version,
                sub_packets,
            }
            | Packet::Minimum {
                version,
                sub_packets,
            }
            | Packet::Maximum {
                version,
                sub_packets,
            }
            | Packet::GreaterThan {
                version,
                sub_packets,
            }
            | Packet::LessThan {
                version,
                sub_packets,
            }
            | Packet::EqualTo {
                version,
                sub_packets,
            } => {
                total += *version as u64;
                queue.extend(sub_packets);
            }
        }
    }
    let part1 = total;

    let part2 = root_packet.value();

    (part1, part2)
}

/// Read data from a an array of bits
struct PacketReader {
    data: Vec<u8>,
    pos: usize,
    length: usize,
}

impl PacketReader {
    fn from_bytes(input: &[u8]) -> Self {
        let mut data = Vec::new();
        for i in (0..input.len()).step_by(2) {
            if i == input.len() - 1 {
                data.push(input[i] & 0b1111);
            } else {
                data.push((input[i] & 0b1111) << 4 | (input[i + 1] & 0b1111));
            }
        }
        Self {
            data,
            pos: 0,
            length: input.len() * 4,
        }
    }

    /// Read a single bit
    fn read_bit(&mut self) -> bool {
        let bit = (self.data[self.pos / 8] >> (7 - (self.pos % 8))) & 1;
        self.pos += 1;
        assert!(self.pos <= self.length);
        bit != 0
    }

    /// Read `n` bits
    fn readn(&mut self, nbits: usize) -> i64 {
        assert!(nbits <= 64);
        let mut result = 0;
        for _ in 0..nbits {
            result <<= 1;
            result |= self.read_bit() as i64;
        }
        result
    }

    /// Read a packet
    fn read_packet(&mut self) -> Packet {
        let version = self.readn(3) as u8;
        let type_id = self.readn(3) as u8;
        match type_id {
            0 => Packet::Sum {
                version,
                sub_packets: self.read_subpackets(),
            },
            1 => Packet::Product {
                version,
                sub_packets: self.read_subpackets(),
            },
            2 => Packet::Minimum {
                version,
                sub_packets: self.read_subpackets(),
            },
            3 => Packet::Maximum {
                version,
                sub_packets: self.read_subpackets(),
            },
            4 => {
                let mut value = 0;
                loop {
                    let next = self.readn(5);
                    value <<= 4;
                    value |= next & 0b1111;
                    if next & 0b10000 == 0 {
                        break;
                    }
                }
                Packet::Literal { version, value }
            }
            5 => Packet::GreaterThan {
                version,
                sub_packets: self.read_subpackets(),
            },
            6 => Packet::LessThan {
                version,
                sub_packets: self.read_subpackets(),
            },
            7 => Packet::EqualTo {
                version,
                sub_packets: self.read_subpackets(),
            },
            _ => unreachable!(),
        }
    }

    fn read_subpackets(&mut self) -> Vec<Packet> {
        let length_type = self.read_bit();
        if !length_type {
            let bit_length = self.readn(15) as usize;
            let expected_pos = self.pos + bit_length;
            let mut sub_packets = Vec::new();
            while self.pos < expected_pos {
                sub_packets.push(self.read_packet());
            }
            assert_eq!(self.pos, expected_pos);
            sub_packets
        } else {
            let sub_count = self.readn(11);
            (0..sub_count).map(|_| self.read_packet()).collect()
        }
    }
}

enum Packet {
    Sum {
        version: u8,
        sub_packets: Vec<Packet>,
    },
    Product {
        version: u8,
        sub_packets: Vec<Packet>,
    },
    Minimum {
        version: u8,
        sub_packets: Vec<Packet>,
    },
    Maximum {
        version: u8,
        sub_packets: Vec<Packet>,
    },
    Literal {
        version: u8,
        value: i64,
    },
    GreaterThan {
        version: u8,
        sub_packets: Vec<Packet>,
    },
    LessThan {
        version: u8,
        sub_packets: Vec<Packet>,
    },
    EqualTo {
        version: u8,
        sub_packets: Vec<Packet>,
    },
}

impl Packet {
    fn value(&self) -> i64 {
        match self {
            Packet::Sum { sub_packets, .. } => sub_packets.iter().map(|p| p.value()).sum(),
            Packet::Product { sub_packets, .. } => sub_packets.iter().map(|p| p.value()).product(),
            Packet::Minimum { sub_packets, .. } => {
                sub_packets.iter().map(|p| p.value()).min().unwrap()
            }
            Packet::Maximum { sub_packets, .. } => {
                sub_packets.iter().map(|p| p.value()).max().unwrap()
            }
            Packet::Literal { value, .. } => *value,
            Packet::GreaterThan { sub_packets, .. } => {
                (sub_packets[0].value() > sub_packets[1].value()) as i64
            }
            Packet::LessThan { sub_packets, .. } => {
                (sub_packets[0].value() < sub_packets[1].value()) as i64
            }
            Packet::EqualTo { sub_packets, .. } => {
                (sub_packets[0].value() == sub_packets[1].value()) as i64
            }
        }
    }
}
