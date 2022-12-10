use itertools::Itertools;

enum Instruction {
    Noop,
    Addx(i32),
}
impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        match value {
            "noop" => Self::Noop,
            _ => match value.split_once(' ').unwrap() {
                ("addx", val) => Self::Addx(val.parse().unwrap()),
                _ => unreachable!(),
            },
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    input
        .lines()
        .map_into::<Instruction>()
        .scan(1, |x, instruction| match instruction {
            Instruction::Noop => {
                *x += 1;
                Some(vec![*x])
            }
            Instruction::Addx(val) => {
                *x += val;
                Some(vec![*x, *x])
            }
        })
        .flatten()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(cycle, x)| x * (cycle + 1) as i32)
        .sum1()
}

pub fn part_two(input: &str) {
    use pathfinding::prelude::Grid;

    let grid = input
        .lines()
        .map_into::<Instruction>()
        .scan(1, |x, instruction| match instruction {
            Instruction::Noop => {
                *x += 1;
                Some(vec![*x])
            }
            Instruction::Addx(val) => {
                *x += val;
                Some(vec![*x, *x])
            }
        })
        .flatten()
        .enumerate()
        .flat_map(|(cycle, x)| {
            (x.abs_diff(cycle as i32 % 40) <= 1)
                .then_some((cycle % 40, cycle / 40))
        })
        .collect::<Grid>();

    format!("{grid:#?}");
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    // advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    // #[test]
    // fn test_part_two() {
    //     let input = advent_of_code::read_file("examples", 10);
    //     assert_eq!(part_two(&input), None);
    // }
}
