use itertools::Itertools;
use std::collections::HashSet;

#[allow(dead_code)]
fn animate_rope(cells: &[(i32, i32)]) {
    let head = cells.first().unwrap();
    let tails = cells.iter().skip(1);

    print!("{}[2J", 27 as char);

    let mut grid = vec![vec!['.'; 40]; 20];
    tails.for_each(|tail| {
        grid[(tail.1 % 20 + 20) as usize % 20][(tail.0 % 40 + 40) as usize % 40] = 'T';
    });
    grid[(head.1 % 20 + 20) as usize % 20][(head.0 % 40 + 40) as usize % 40] = 'H';

    grid.iter().for_each(|row| {
        row.iter().for_each(|cell| print!("{}", cell));
        println!();
    });

    std::thread::sleep(std::time::Duration::from_millis(50));
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "L" => Self::Left,
            "R" => Self::Right,
            "U" => Self::Up,
            "D" => Self::Down,
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
        Self { direction: direction.into(), step_count: step_count.parse().unwrap() }
    }
}

fn simulate_motions(motions: impl Iterator<Item = Motion>, rope_length: usize) -> usize {
    let mut tails = vec![(0_i32, 0_i32); rope_length + 1];

    motions
        .fold(HashSet::new(), |mut acc, Motion { direction, step_count }| {
            (0..step_count).for_each(|_| {
                let mut head = tails.first_mut().unwrap();
                match direction {
                    Direction::Left => head.0 -= 1,
                    Direction::Right => head.0 += 1,
                    Direction::Up => head.1 -= 1,
                    Direction::Down => head.1 += 1,
                }

                tails.iter_mut().reduce(|head, tail| {
                    let (x, y) = (head.0 - tail.0, head.1 - tail.1);

                    if x.abs() > 1 || y.abs() > 1 {
                        *tail = (tail.0 + x.signum(), tail.1 + y.signum());
                    }

                    tail
                });

                acc.insert(tails.last().cloned().unwrap());

                // animate_rope(&tails);
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
