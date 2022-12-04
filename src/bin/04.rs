use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let overlaps_count: u32 = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .flat_map(|(first, second)| first.split('-').chain(second.split('-')))
        .map(|range| range.parse::<u32>().unwrap())
        .tuples()
        .filter(|(first_start, first_end, second_start, second_end)| {
            let first_range = *first_start..=*first_end;
            let second_range = *second_start..=*second_end;
            (first_range.contains(second_start)
                && first_range.contains(second_end))
                || (second_range.contains(first_start)
                    && second_range.contains(first_end))
        })
        .count()
        .try_into()
        .unwrap();

    Some(overlaps_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let overlaps_count: u32 = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .flat_map(|(first, second)| first.split('-').chain(second.split('-')))
        .map(|range| range.parse::<u32>().unwrap())
        .tuples()
        .filter(|(first_start, first_end, second_start, second_end)| {
            let first_range = *first_start..=*first_end;
            let second_range = *second_start..=*second_end;
            first_range.contains(second_start)
                || second_range.contains(first_start)
        })
        .count()
        .try_into()
        .unwrap();

    Some(overlaps_count)
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
