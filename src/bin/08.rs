#![feature(array_windows)]

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Tree {
    position: Position,
    height: u32,
}

#[derive(Debug)]
struct Grid {
    trees: Vec<Vec<Tree>>,
}

impl Grid {
    fn from(input: &str) -> Self {
        let trees = input
            .lines()
            .enumerate()
            .map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .map(|(y, height)| Tree {
                        height: height.to_digit(10).unwrap(),
                        position: Position { x, y },
                    })
                    .collect_vec()
            })
            .collect_vec();

        Self { trees }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
enum FromDirection {
    Left,
    Right,
    Up,
    Down,
}

pub fn part_one(input: &str) -> Option<u32> {
    let Grid { trees } = Grid::from(input);
    let mut visible: HashSet<(&Tree, FromDirection)> = HashSet::new();

    // by row
    trees[1..trees.len() - 1].iter().for_each(|row| {
        // left to right
        row[1..row.len() - 1].iter().fold(&row[0], |highest, tree| {
            if tree.height > highest.height {
                visible.insert((tree, FromDirection::Left));
                return tree;
            }
            highest
        });
        // right to left
        row[1..row.len() - 1].iter().rev().fold(
            &row[row.len() - 1],
            |highest, tree| {
                if tree.height > highest.height {
                    visible.insert((tree, FromDirection::Right));
                    return tree;
                }
                highest
            },
        );
    });

    // by column
    (1..trees[0].len() - 1).for_each(|col| {
        // top to bottom
        trees[1..trees.len() - 1].iter().fold(
            &trees[0][col],
            |highest, row| {
                if row[col].height > highest.height {
                    visible.insert((&row[col], FromDirection::Up));
                    return &row[col];
                }
                highest
            },
        );
        // bottom to top
        trees[1..trees.len() - 1].iter().rev().fold(
            &trees[trees.len() - 1][col],
            |highest, row| {
                if row[col].height > highest.height {
                    visible.insert((&row[col], FromDirection::Down));
                    return &row[col];
                }
                highest
            },
        );
    });

    let total_visible_trees =
        visible.iter().unique_by(|(tree, _)| &tree.position).count()
            + trees.len() * 2
            + (trees[0].len() - 2) * 2;

    Some(total_visible_trees as u32)
}

fn scenic_score(trees: &Vec<Vec<Tree>>, x: usize, y: usize) -> u32 {
    // left
    let from_left = trees[x][0..y]
        .iter()
        .rev()
        .fold_while(0, |score, tree| {
            if tree.height < trees[x][y].height {
                return Continue(score + 1);
            } else if tree.height >= trees[x][y].height {
                return Done(score + 1);
            }
            Done(score)
        })
        .into_inner();

    // right
    let from_right = trees[x][y + 1..trees[0].len()]
        .iter()
        .fold_while(0, |score, tree| {
            if tree.height < trees[x][y].height {
                return Continue(score + 1);
            } else if tree.height >= trees[x][y].height {
                return Done(score + 1);
            }

            Done(score)
        })
        .into_inner();

    // up
    let from_up = trees[0..x]
        .iter()
        .rev()
        .fold_while(0, |score, row| {
            if row[y].height < trees[x][y].height {
                return Continue(score + 1);
            } else if row[y].height >= trees[x][y].height {
                return Done(score + 1);
            }
            Done(score)
        })
        .into_inner();

    // down
    let from_down = trees[x + 1..trees.len()]
        .iter()
        .fold_while(0, |score, row| {
            if row[y].height < trees[x][y].height {
                return Continue(score + 1);
            } else if row[y].height >= trees[x][y].height {
                return Done(score + 1);
            }
            Done(score)
        })
        .into_inner();

    from_left * from_right * from_down * from_up
}

pub fn part_two(input: &str) -> Option<u32> {
    let Grid { trees } = Grid::from(input);

    (0..trees.len())
        .cartesian_product(0..trees[0].len())
        .map(|(x, y)| scenic_score(&trees, x, y))
        .max()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
