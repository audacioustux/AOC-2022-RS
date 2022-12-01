pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut best_sum = 0;

    for line in input.lines() {
        if line.is_empty() {
            sum = 0;
            continue;
        }

        let n: u32 = line.to_string().parse().unwrap();
        sum += n;
        if sum > best_sum {
            best_sum = sum;
        }
    }

    Some(best_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::new();
    let mut sum = 0;

    for line in input.lines() {
        if line.is_empty() {
            heap.push(sum);
            sum = 0;
            continue;
        }
        let n: u32 = line.to_string().parse().unwrap();
        sum += n;
    }
    heap.push(sum);

    let total_sum = heap.iter().take(3).sum();

    Some(total_sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
