use itertools::Itertools;

fn find_marker(buffer: &str, num_of_unique_char: u32) -> Option<u32> {
    buffer
        .chars()
        .collect::<Vec<char>>()
        .windows(num_of_unique_char as usize)
        .find_position(|s| s.iter().all_unique())
        .map(|(i, _)| i as u32 + num_of_unique_char)
}

pub fn part_one(input: &str) -> Option<u32> {
    find_marker(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    find_marker(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
