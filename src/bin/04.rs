use std::ops::RangeInclusive;

use itertools::Itertools;

fn get_assignments(
    input: &str,
) -> impl Iterator<Item = (RangeInclusive<u32>, RangeInclusive<u32>)> + '_ {
    input.lines().map(|pair| pair.splitn(2, ',')).map(|ranges| {
        ranges
            .flat_map(|range| {
                range
                    .split('-')
                    .map(|n| n.parse().unwrap())
                    .tuples()
                    .map(|(start, end)| start..=end)
            })
            .collect_tuple()
            .unwrap()
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    get_assignments(input)
        .filter(|(first, second)| {
            (first.contains(second.start()) && first.contains(second.end()))
                || (second.contains(first.start())
                    && second.contains(first.end()))
        })
        .count()
        .try_into()
        .ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    get_assignments(input)
        .filter(|(first, second)| {
            first.contains(second.start()) || second.contains(first.start())
        })
        .count()
        .try_into()
        .ok()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
