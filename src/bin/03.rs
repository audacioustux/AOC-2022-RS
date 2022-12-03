use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(first, second)| {
            first
                .chars()
                .sorted()
                .dedup()
                .cartesian_product(second.chars().sorted().dedup())
                .find(|(first, second)| first == second)
                .map(|(item, _)| match item {
                    'a'..='z' => item as u32 - 'a' as u32 + 1,
                    'A'..='Z' => item as u32 - 'A' as u32 + 27,
                    _ => unreachable!(),
                })
                .unwrap()
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum: u32 = input
        .lines()
        .tuples()
        .map(|(first, second, third)| {
            first
                .chars()
                .sorted()
                .dedup()
                .cartesian_product(second.chars().sorted().dedup())
                .cartesian_product(third.chars().sorted().dedup())
                .find(|((first, second), third)| {
                    first == second && second == third
                })
                .map(|((item, _), _)| match item {
                    'a'..='z' => item as u32 - 'a' as u32 + 1,
                    'A'..='Z' => item as u32 - 'A' as u32 + 27,
                    _ => unreachable!(),
                })
                .unwrap()
        })
        .sum();

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
