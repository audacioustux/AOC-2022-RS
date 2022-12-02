use itertools::Itertools;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}
impl Shape {
    fn value(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3,
        }
    }
    fn from_outcome(opponent: &Shape, outcome: &Outcome) -> Self {
        use Outcome::*;
        use Shape::*;

        match (opponent, outcome) {
            (shape @ _, Draw) => *shape,
            (Rock, Win) => Paper,
            (Rock, Loss) => Scissor,
            (Paper, Win) => Scissor,
            (Paper, Loss) => Rock,
            (Scissor, Win) => Rock,
            (Scissor, Loss) => Paper,
        }
    }
}
impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissor),
            _ => Err(()),
        }
    }
}

enum Outcome {
    Loss,
    Draw,
    Win,
}
impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}
impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }

    fn evaluate(opponent: &Shape, player: &Shape) -> Self {
        use Shape::*;

        match (opponent, player) {
            (Rock, Rock) => Self::Draw,
            (Rock, Paper) => Self::Win,
            (Rock, Scissor) => Self::Loss,
            (Paper, Paper) => Self::Draw,
            (Paper, Rock) => Self::Loss,
            (Paper, Scissor) => Self::Win,
            (Scissor, Scissor) => Self::Draw,
            (Scissor, Rock) => Self::Win,
            (Scissor, Paper) => Self::Loss,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .filter_map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<Shape>().unwrap())
                .collect_tuple::<(_, _)>()
        })
        .map(|(opponent, player)| Outcome::evaluate(&opponent, &player).score() + player.value())
        .sum();

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .filter_map(|line| {
            line.split_ascii_whitespace()
                .collect_tuple::<(_, _)>()
                .map(|(shape, outcome)| {
                    (
                        shape.parse::<Shape>().unwrap(),
                        outcome.parse::<Outcome>().unwrap(),
                    )
                })
        })
        .map(|(opponent, outcome)| {
            Shape::from_outcome(&opponent, &outcome).value() + outcome.score()
        })
        .sum();

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
