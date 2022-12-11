use itertools::Itertools;

fn parse<'a>(input: &mut impl Iterator<Item = &'a str>) -> Vec<u32> {
    let (mut total, mut subdirs) = (0, vec![]);
    loop {
        match input.next().map(|s| s.split_whitespace().collect_vec()).as_deref() {
            Some(["$", "cd", ".."]) | None => break,
            Some(["$", "cd", dir]) if *dir != "/" => {
                subdirs.extend(parse(input));
                total += subdirs.last().unwrap();
            }
            Some([s, _]) if !["$", "dir"].contains(s) => {
                total += s.parse::<u32>().unwrap();
            }
            _ => (),
        }
    }
    subdirs.push(total);

    subdirs
}

pub fn part_one(input: &str) -> Option<u32> {
    let total = parse(&mut input.lines()).into_iter().filter(|&size| size <= 100_000).sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sizes = parse(&mut input.lines());
    sizes.sort_unstable();

    let unused = 70_000_000 - sizes.last().unwrap();
    sizes.into_iter().find(|&size| size >= 30_000_000 - unused)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
