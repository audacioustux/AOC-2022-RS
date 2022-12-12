use pathfinding::prelude::{bfs, Matrix};

fn parse(input: &str) -> (Matrix<u8>, (usize, usize), (usize, usize)) {
    let mut matrix = Matrix::from_rows(input.lines().map(str::bytes)).unwrap();
    let start = matrix.indices().find(|pos| matrix[*pos] == b'S').unwrap();
    let end = matrix.indices().find(|pos| matrix[*pos] == b'E').unwrap();

    matrix[start] = b'a';
    matrix[end] = b'z';

    (matrix, start, end)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (ref matrix, start, end) = parse(input);

    bfs(
        &start,
        |&curr| matrix.neighbours(curr, false).filter(move |&nei| matrix[nei] <= matrix[curr] + 1),
        |curr| *curr == end,
    )
    .map(|steps| steps.len() - 1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (ref matrix, _, end) = parse(input);

    bfs(
        &end,
        |&curr| matrix.neighbours(curr, false).filter(move |&nei| matrix[curr] <= matrix[nei] + 1),
        |curr| matrix[*curr] == b'a',
    )
    .map(|steps| steps.len() - 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
