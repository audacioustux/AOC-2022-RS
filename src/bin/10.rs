use itertools::Itertools;

fn parse(input: &str) -> impl Iterator<Item = i32> + '_ {
    input
        .lines()
        .scan(1, |x, instruction| match instruction {
            "noop" => Some(vec![*x]),
            _ => match instruction.split_once(' ').unwrap() {
                ("addx", val) => {
                    let x_ = *x;
                    *x += val.parse::<i32>().unwrap();

                    Some(vec![x_, x_])
                }
                _ => unreachable!(),
            },
        })
        .flatten()
}
pub fn part_one(input: &str) -> Option<i32> {
    parse(input)
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(cycle, x)| x * (cycle + 1) as i32)
        .sum1()
}

pub fn part_two(input: &str) -> Option<String> {
    use pathfinding::prelude::Grid;

    let grid = parse(input)
        .enumerate()
        .flat_map(|(cycle, x)| {
            (x.abs_diff(cycle as i32 % 40) <= 1)
                .then_some((cycle % 40, cycle / 40))
        })
        .collect::<Grid>();

    Some(format!("{grid:#?}"))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
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
