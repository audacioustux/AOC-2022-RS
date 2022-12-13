use itertools::Itertools;
use std::cmp::Ordering::*;
use std::collections::VecDeque;

#[derive(Eq, PartialEq, Clone, Copy)]
enum PacketSymbol {
    LeftBracket,
    RightBracket,
    Integer(u32),
}

type Packet = VecDeque<PacketSymbol>;

fn parse_packet(packet: &str) -> Packet {
    use PacketSymbol::*;

    let mut packet_symbols = VecDeque::with_capacity(packet.len());
    let mut integer_string = String::with_capacity(2);

    for symbol in packet.chars() {
        if !symbol.is_ascii_digit() {
            if !integer_string.is_empty() {
                packet_symbols.push_back(Integer(integer_string.parse().unwrap()));
                integer_string.clear();
            }

            match symbol {
                '[' => packet_symbols.push_back(LeftBracket),
                ']' => packet_symbols.push_back(RightBracket),
                ',' => (),
                _ => unreachable!(),
            }
        } else {
            integer_string.push(symbol);
        }
    }

    packet_symbols
}

fn is_right_order(left: &Packet, right: &Packet) -> bool {
    use PacketSymbol::*;

    let mut left = left.to_owned();
    let mut right = right.to_owned();

    while !left.is_empty() && !right.is_empty() {
        let left_symbol = left.pop_front().unwrap();
        let right_symbol = right.pop_front().unwrap();

        match (left_symbol, right_symbol) {
            (left_symbol, right_symbol) if left_symbol == right_symbol => (),
            (_, RightBracket) => return false,
            (RightBracket, _) => return true,
            (Integer(left_integer), Integer(right_integer)) => {
                match left_integer.cmp(&right_integer) {
                    Less => return true,
                    Greater => return false,
                    _ => (),
                }
            }
            (LeftBracket, Integer(integer)) => {
                right.push_front(RightBracket);
                right.push_front(Integer(integer));
            }
            (Integer(integer), LeftBracket) => {
                left.push_front(RightBracket);
                left.push_front(Integer(integer));
            }
            _ => unreachable!(),
        }
    }

    true
}

fn parse(input: &str) -> impl Iterator<Item = (Packet, Packet)> + '_ {
    input.split("\n\n").map(|packet_pair| {
        let mut packets = packet_pair.lines().map(parse_packet);
        (packets.next().unwrap(), packets.next().unwrap())
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    parse(input)
        .enumerate()
        .filter(|(_, (left, right))| is_right_order(left, right))
        .map(|(index, _)| index + 1)
        .sum1()
}

pub fn part_two(input: &str) -> Option<usize> {
    let packets = parse(input).flat_map(|(left, right)| [left, right]).collect_vec();

    let indice_1 = packets.iter().filter(|p| is_right_order(p, &parse_packet("[[2]]"))).count() + 1;
    let indice_2 = packets.iter().filter(|p| is_right_order(p, &parse_packet("[[6]]"))).count() + 2;

    Some(indice_1 * indice_2)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
