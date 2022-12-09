use itertools::Itertools;
use std::collections::HashSet;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => unreachable!(),
        }
    }
}

struct Motion {
    direction: Direction,
    step_count: usize,
}
impl From<&str> for Motion {
    fn from(value: &str) -> Self {
        let (direction, step_count) = value.split_once(' ').unwrap();
        Self {
            direction: direction.into(),
            step_count: step_count.parse().unwrap(),
        }
    }
}

fn simulate_motions(
    motions: impl Iterator<Item = Motion>,
    rope_length: usize,
) -> usize {
    let mut tails = vec![(0_i32, 0_i32); rope_length + 1];

    motions
        .fold(HashSet::new(), |mut acc, motion| {
            (0..motion.step_count).for_each(|_| {
                let mut head = tails.first_mut().unwrap();
                match motion.direction {
                    Direction::Up => head.1 -= 1,
                    Direction::Down => head.1 += 1,
                    Direction::Left => head.0 -= 1,
                    Direction::Right => head.0 += 1,
                }

                let mut head = tails.first().cloned().unwrap();

                tails.iter_mut().skip(1).for_each(|tail| {
                    let (x, y) = (head.0 - tail.0, head.1 - tail.1);

                    if x.abs() > 1 || y.abs() > 1 {
                        *tail = (tail.0 + x.signum(), tail.1 + y.signum());
                    }

                    head = *tail;
                });

                acc.insert(tails.last().cloned().unwrap());
            });

            acc
        })
        .len()
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.lines().map_into::<Motion>();
    simulate_motions(input, 1).try_into().ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.lines().map_into::<Motion>();
    simulate_motions(input, 9).try_into().ok()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }

    #[test]
    fn test_part_two_2() {
        use indoc::indoc;
        let input = indoc! {"
        R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20
        "}
        .trim();

        assert_eq!(part_two(&input), Some(36));
    }
}
