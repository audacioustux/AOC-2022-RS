use itertools::Itertools;
use std::collections::HashSet;
use std::iter::{once, repeat};

fn solve(input: &str, is_abyss: bool) -> usize {
    once((parse(input), (500, 0))).fold(0, |_, ((mut solid, bottom), sand)| {
        repeat(sand)
            .take_while(|(mut sx, mut sy)| {
                while sy != bottom + 1 {
                    match [(0, 1), (-1, 1), (1, 1)]
                        .iter()
                        .find(|(dx, dy)| !solid.contains(&(sx + dx, sy + dy)))
                    {
                        Some((dx, dy)) => (sx, sy) = (sx + dx, sy + dy),
                        _ => break,
                    }
                }

                if is_abyss && sy == bottom + 1 {
                    false
                } else {
                    solid.insert((sx, sy))
                }
            })
            .count()
    })
}

fn parse(input: &str) -> (HashSet<(i32, i32)>, i32) {
    input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .flat_map(|coords| {
                    coords
                        .split_once(',')
                        .and_then(|(x, y)| Some((x.parse::<i32>().ok()?, y.parse::<i32>().ok()?)))
                })
                .collect_vec()
        })
        .fold((HashSet::new(), 0), |(r, b), p| {
            p.windows(2).flat_map(|w| TryInto::<[_; 2]>::try_into(w).ok()).fold(
                (r, b),
                |(mut rocks, mut bottom), [(a, b), (c, d)]| {
                    for y in b.min(d)..=b.max(d) {
                        for x in a.min(c)..=a.max(c) {
                            rocks.extend(std::iter::once((x, y)))
                        }

                        bottom = bottom.max(y)
                    }

                    (rocks, bottom)
                },
            )
        })
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, true))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(input, false))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
