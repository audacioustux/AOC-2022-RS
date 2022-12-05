use std::collections::HashMap;

use itertools::Itertools;

fn parse_stacks(input: &str) -> HashMap<usize, Vec<char>> {
    let levels = input.lines().rev();

    levels
        .skip(1)
        .map(|level| level.chars().skip(1).step_by(4))
        .fold(HashMap::new(), |mut stacks, crates| {
            crates
                .enumerate()
                .filter(|(_, c)| c.is_ascii_alphabetic())
                .for_each(|(i, crate_id)| {
                    stacks.entry(i + 1).or_insert_with(Vec::new).push(crate_id)
                });

            stacks
        })
}

struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}
fn parse_moves(input: &str) -> impl Iterator<Item = Move> + '_ {
    input.lines().flat_map(|line| {
        line.split_ascii_whitespace().collect_tuple().map(
            |(_, quantity, _, from, _, to)| Move {
                quantity: quantity.parse().unwrap(),
                from: from.parse().unwrap(),
                to: to.parse().unwrap(),
            },
        )
    })
}

struct CrateMover<'a>(&'a str);

fn move_crates(
    stacks: &mut HashMap<usize, Vec<char>>,
    moves: impl Iterator<Item = Move>,
    crate_mover: CrateMover,
) -> String {
    moves.for_each(|Move { quantity, from, to }| {
        let crates = {
            let from = stacks.get_mut(&from).unwrap();
            let crates = from.drain((from.len() - quantity)..from.len());

            match crate_mover {
                CrateMover("9000") => crates.rev().collect_vec(),
                CrateMover("9001") => crates.collect_vec(),
                _ => unreachable!(),
            }
        };
        stacks.entry(to).and_modify(|stack| stack.extend(crates));
    });

    (1..=stacks.len())
        .map(|i| stacks.get(&i).unwrap().last().unwrap())
        .join("")
}

pub fn part_one(input: &str) -> Option<String> {
    let (stacks, moves) = input.split_once("\n\n").unwrap();

    let mut stacks = parse_stacks(stacks);
    let moves = parse_moves(moves);

    Some(move_crates(&mut stacks, moves, CrateMover("9000")))
}

pub fn part_two(input: &str) -> Option<String> {
    let (stacks, moves) = input.split_once("\n\n").unwrap();

    let mut stacks = parse_stacks(stacks);
    let moves = parse_moves(moves);

    Some(move_crates(&mut stacks, moves, CrateMover("9001")))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".into()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".into()));
    }
}
