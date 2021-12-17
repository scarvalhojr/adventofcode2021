use std::convert::TryFrom;
use std::str::FromStr;
use Packet::*;
use PacketType::*;

#[derive(Clone)]
pub struct Message(Vec<char>);

type PacketVersion = u64;

#[derive(Debug, Eq, PartialEq)]
enum PacketType {
    OperSum,
    OperProduct,
    OperMinimum,
    OperMaximum,
    Literal,
    OperGreaterThan,
    OperLessThan,
    OperEqualTo,
}

impl TryFrom<u64> for PacketType {
    type Error = ();

    fn try_from(v: u64) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(OperSum),
            1 => Ok(OperProduct),
            2 => Ok(OperMinimum),
            3 => Ok(OperMaximum),
            4 => Ok(Literal),
            5 => Ok(OperGreaterThan),
            6 => Ok(OperLessThan),
            7 => Ok(OperEqualTo),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    LiteralValue(PacketVersion, u64),
    Operator(PacketVersion, PacketType, Vec<Packet>),
}

impl Packet {
    fn sum_versions(&self) -> u64 {
        match self {
            LiteralValue(version, _) => *version,
            Operator(version, _, packets) => {
                *version + packets.iter().map(Packet::sum_versions).sum::<u64>()
            }
        }
    }

    fn value(&self) -> Option<u64> {
        match self {
            LiteralValue(_, value) => Some(*value),
            Operator(_, OperSum, packets) => {
                packets.iter().map(Packet::value).sum()
            }
            Operator(_, OperProduct, packets) => {
                packets.iter().map(Packet::value).product()
            }
            Operator(_, OperMinimum, packets) => {
                packets.iter().map(Packet::value).min().flatten()
            }
            Operator(_, OperMaximum, packets) => {
                packets.iter().map(Packet::value).max().flatten()
            }
            Operator(_, OperGreaterThan, packets) => match &packets[..] {
                [p1, p2] => {
                    if p1.value()? > p2.value()? {
                        Some(1)
                    } else {
                        Some(0)
                    }
                }
                _ => None,
            },
            Operator(_, OperLessThan, packets) => match &packets[..] {
                [p1, p2] => {
                    if p1.value()? < p2.value()? {
                        Some(1)
                    } else {
                        Some(0)
                    }
                }
                _ => None,
            },
            Operator(_, OperEqualTo, packets) => match &packets[..] {
                [p1, p2] => {
                    if p1.value()? == p2.value()? {
                        Some(1)
                    } else {
                        Some(0)
                    }
                }
                _ => None,
            },
            _ => unreachable!(),
        }
    }
}

impl Message {
    fn get_packet(&mut self) -> Option<Packet> {
        let (packet, _) = self.get_inner_packet()?;
        self.drop_padding()?;
        Some(packet)
    }

    fn get_inner_packet(&mut self) -> Option<(Packet, usize)> {
        let version = self.pop_bits(3)?;
        let type_id = self.pop_bits(3)?;
        let mut num_bits = 6;

        let packet = match type_id.try_into() {
            Ok(Literal) => {
                let (literal, literal_bits) = self.get_literal()?;
                num_bits += literal_bits;
                LiteralValue(version, literal)
            }
            Ok(packet_type) => {
                let (sub_packets, sub_packet_bits) = self.get_sub_packets()?;
                num_bits += sub_packet_bits;
                Operator(version, packet_type, sub_packets)
            }
            _ => {
                return None;
            }
        };

        Some((packet, num_bits))
    }

    fn get_sub_packets(&mut self) -> Option<(Vec<Packet>, usize)> {
        let mut num_bits = 0;
        let mut sub_packets = Vec::new();
        let length_type_id = self.pop_bits(1)?;

        match length_type_id {
            0 => {
                let total_bit_len = self.pop_bits(15)?.try_into().ok()?;
                while num_bits < total_bit_len {
                    let (packet, sub_bits) = self.get_inner_packet()?;
                    num_bits += sub_bits;
                    sub_packets.push(packet);
                }
                if num_bits != total_bit_len {
                    return None;
                }
                num_bits += 15;
            }
            1 => {
                let total_sub_packets = self.pop_bits(11)?;
                num_bits += 11;
                for _ in 1..=total_sub_packets {
                    let (packet, sub_bits) = self.get_inner_packet()?;
                    num_bits += sub_bits;
                    sub_packets.push(packet);
                }
            }
            _ => {
                return None;
            }
        }

        Some((sub_packets, 1 + num_bits))
    }

    fn get_literal(&mut self) -> Option<(u64, usize)> {
        let mut num_bits = 0;
        let mut literal = 0;
        let mut keep_reading = 1;
        while keep_reading == 1 {
            keep_reading = self.pop_bits(1)?;
            literal = literal << 4 | self.pop_bits(4)?;
            num_bits += 5;
        }
        Some((literal, num_bits))
    }

    fn pop_bits(&mut self, num_bits: usize) -> Option<u64> {
        if num_bits > self.0.len() {
            return None;
        }

        let bits = self.0.split_off(self.0.len() - num_bits);
        u64::from_str_radix(
            bits.into_iter().rev().collect::<String>().as_str(),
            2,
        )
        .ok()
    }

    fn drop_padding(&mut self) -> Option<()> {
        if self.0.is_empty() || self.pop_bits(self.0.len())? == 0 {
            Some(())
        } else {
            None
        }
    }
}

pub fn part1(message: &Message) -> Option<u64> {
    message
        .clone()
        .get_packet()
        .map(|packet| packet.sum_versions())
}

pub fn part2(message: &Message) -> Option<u64> {
    message
        .clone()
        .get_packet()
        .and_then(|packet| packet.value())
}

impl FromStr for Message {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim()
            .chars()
            .rev()
            .map(|ch| {
                ch.to_digit(16).ok_or_else(|| {
                    format!("Invalid hexadecimal character '{}'", ch)
                })
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|vec| {
                Self(
                    vec.into_iter()
                        .flat_map(|num| {
                            format!("{:04b}", num)
                                .chars()
                                .rev()
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<char>>(),
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal() {
        let mut message: Message = "D2FE28".parse().unwrap();
        let packet = message.get_packet().unwrap();
        assert_eq!(packet, LiteralValue(6, 2021));
        assert_eq!(packet.sum_versions(), 6);
        assert_eq!(packet.value(), Some(2021));
    }

    #[test]
    fn operator_less_than() {
        let mut message: Message = "38006F45291200".parse().unwrap();
        let packet = message.get_packet().unwrap();
        assert_eq!(
            packet,
            Operator(
                1,
                OperLessThan,
                vec![LiteralValue(6, 10), LiteralValue(2, 20)]
            )
        );
        assert_eq!(packet.sum_versions(), 9);
        assert_eq!(packet.value(), Some(1));
    }

    #[test]
    fn operator_maximum() {
        let mut message: Message = "EE00D40C823060".parse().unwrap();
        let packet = message.get_packet().unwrap();
        assert_eq!(
            packet,
            Operator(
                7,
                OperMaximum,
                vec![
                    LiteralValue(2, 1),
                    LiteralValue(4, 2),
                    LiteralValue(1, 3)
                ]
            )
        );
        assert_eq!(packet.sum_versions(), 14);
        assert_eq!(packet.value(), Some(3));
    }

    #[test]
    fn operator_minimum() {
        let mut message: Message = "8A004A801A8002F478".parse().unwrap();
        let packet = message.get_packet().unwrap();
        assert_eq!(
            packet,
            Operator(
                4,
                OperMinimum,
                vec![Operator(
                    1,
                    OperMinimum,
                    vec![Operator(5, OperMinimum, vec![LiteralValue(6, 15)])]
                )]
            )
        );
        assert_eq!(packet.sum_versions(), 16);
        assert_eq!(packet.value(), Some(15));
    }

    #[test]
    fn operator_sum() {
        let mut message: Message =
            "620080001611562C8802118E34".parse().unwrap();
        let packet = message.get_packet().unwrap();
        assert_eq!(
            packet,
            Operator(
                3,
                OperSum,
                vec![
                    Operator(
                        0,
                        OperSum,
                        vec![LiteralValue(0, 10), LiteralValue(5, 11)]
                    ),
                    Operator(
                        1,
                        OperSum,
                        vec![LiteralValue(0, 12), LiteralValue(3, 13)]
                    )
                ]
            )
        );
        assert_eq!(packet.sum_versions(), 12);
        assert_eq!(packet.value(), Some(46));
    }

    #[test]
    fn operator_sum2() {
        let mut message: Message =
            "C0015000016115A2E0802F182340".parse().unwrap();
        let packet = message.get_packet().unwrap();
        assert_eq!(
            packet,
            Operator(
                6,
                OperSum,
                vec![
                    Operator(
                        0,
                        OperSum,
                        vec![LiteralValue(0, 10), LiteralValue(6, 11)]
                    ),
                    Operator(
                        4,
                        OperSum,
                        vec![LiteralValue(7, 12), LiteralValue(0, 13)]
                    )
                ]
            )
        );
        assert_eq!(packet.sum_versions(), 23);
        assert_eq!(packet.value(), Some(46));
    }

    #[test]
    fn operator_sum3() {
        let mut message: Message =
            "A0016C880162017C3686B18A3D4780".parse().unwrap();
        let packet = message.get_packet().unwrap();
        assert_eq!(
            packet,
            Operator(
                5,
                OperSum,
                vec![Operator(
                    1,
                    OperSum,
                    vec![Operator(
                        3,
                        OperSum,
                        vec![
                            LiteralValue(7, 6),
                            LiteralValue(6, 6),
                            LiteralValue(5, 12),
                            LiteralValue(2, 15),
                            LiteralValue(2, 15)
                        ]
                    )]
                )]
            )
        );
        assert_eq!(packet.sum_versions(), 31);
        assert_eq!(packet.value(), Some(54));
    }

    #[test]
    fn operator_sum4() {
        let mut message: Message = "C200B40A82".parse().unwrap();
        let packet = message.get_packet().unwrap();
        assert_eq!(packet.value(), Some(3));
    }

    #[test]
    fn operator_product() {
        let mut message: Message = "04005AC33890".parse().unwrap();
        let packet = message.get_packet().unwrap();
        assert_eq!(packet.value(), Some(54));
    }

    #[test]
    fn operator_minimum2() {
        let mut message: Message = "880086C3E88112".parse().unwrap();
        let packet = message.get_packet().unwrap();
        assert_eq!(packet.value(), Some(7));
    }

    #[test]
    fn operator_maximum2() {
        let mut message: Message = "CE00C43D881120".parse().unwrap();
        let packet = message.get_packet().unwrap();
        assert_eq!(packet.value(), Some(9));
    }

    #[test]
    fn operator_less_than2() {
        let mut message: Message = "D8005AC2A8F0".parse().unwrap();
        let packet = message.get_packet().unwrap();
        assert_eq!(packet.value(), Some(1));
    }

    #[test]
    fn operator_greater_than() {
        let mut message: Message = "F600BC2D8F".parse().unwrap();
        let packet = message.get_packet().unwrap();
        assert_eq!(packet.value(), Some(0));
    }

    #[test]
    fn operator_equal_to() {
        let mut message: Message = "9C005AC2F8F0".parse().unwrap();
        let packet = message.get_packet().unwrap();
        assert_eq!(packet.value(), Some(0));
    }

    #[test]
    fn operator_sum_equal_to_product() {
        let mut message: Message =
            "9C0141080250320F1802104A08".parse().unwrap();
        let packet = message.get_packet().unwrap();
        assert_eq!(packet.value(), Some(1));
    }
}
