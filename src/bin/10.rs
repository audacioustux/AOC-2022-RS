use itertools::Itertools;

enum Instructuion {
    Noop,
    Addx(i32),
}
impl From<&str> for Instructuion {
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

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map_into::<Instructuion>()
        .fold(((1, 1), 0), |state, instruction| {
            let ((mut cycle, mut x), mut sum) = state;

            match instruction {
                Instructuion::Noop => {
                    cycle += 1;

                    if (20..=240).step_by(40).contains(&cycle) {
                        sum += x * cycle;
                    }
                }
                Instructuion::Addx(val) => {
                    (0..2).for_each(|c| {
                        cycle += 1;

                        if c == 1 {
                            x += val;
                        };

                        if (20..=240).step_by(40).contains(&cycle) {
                            sum += x * cycle;
                        }
                    });
                }
            }

            ((cycle, x), sum)
        })
        .1
        .try_into()
        .ok()
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
