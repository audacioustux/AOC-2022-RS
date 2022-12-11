use itertools::Itertools;

#[derive(Debug, Clone)]
enum Op {
    Mul(usize),
    Add(usize),
    Squar,
}
impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "* old" => Self::Squar,
            _ => {
                let (op, v) = value.split_once(' ').unwrap();
                match op {
                    "+" => Self::Add(v.parse().unwrap()),
                    "*" => Self::Mul(v.parse().unwrap()),
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Op,
    test: usize,
    if_true: usize,
    if_false: usize,
}
impl From<&str> for Monkey {
    fn from(value: &str) -> Self {
        value
            .lines()
            .skip(1)
            .map(str::trim)
            .collect_tuple()
            .map(|(l1, l2, l3, l4, l5)| Monkey {
                items: l1["Starting items: ".len()..]
                    .split(", ")
                    .map(|x| x.parse().unwrap())
                    .collect(),
                operation: Into::<Op>::into(&l2["Operation: new = old ".len()..]),
                test: l3["Test: divisible by ".len()..].parse().unwrap(),
                if_true: l4["If true: throw to monkey ".len()..].parse().unwrap(),
                if_false: l5["If false: throw to monkey ".len()..].parse().unwrap(),
            })
            .unwrap()
    }
}

fn simulate(mut monkes: Vec<Monkey>, rounds: usize, f: impl Fn(usize) -> usize) -> usize {
    let mut inspections = vec![0; monkes.len()];

    for _ in 0..rounds {
        for monke_id in 0..monkes.len() {
            let Monkey { items, operation, test, if_true, if_false } = monkes[monke_id].clone();

            for item in items {
                let worry = match operation {
                    Op::Add(v) => f(item + v),
                    Op::Mul(v) => f(item * v),
                    Op::Squar => f(item * item),
                };

                let target = if worry % test == 0 { if_true } else { if_false };
                monkes[target].items.push(worry);
            }

            inspections[monke_id] += monkes[monke_id].items.len();
            monkes[monke_id].items.clear();
        }
    }

    inspections.sort_unstable_by_key(|&x| -(x as isize));
    inspections[0..=1].iter().product()
}

pub fn part_one(input: &str) -> Option<usize> {
    let monkes = input.split("\n\n").map_into::<Monkey>();

    Some(simulate(monkes.collect_vec(), 20, |x| x / 3))
}

pub fn part_two(input: &str) -> Option<usize> {
    let monkes = input.split("\n\n").map_into::<Monkey>().collect_vec();
    let mod_by: usize = monkes.clone().into_iter().map(|m| m.test).product();

    Some(simulate(monkes, 10000, |x| x % mod_by))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
