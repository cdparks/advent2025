use sscanf::scanf;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let chunks: Vec<&str> = input.split("\n\n").collect();
    let (regions, shapes) = chunks.split_last()?;

    let shapes: Vec<usize> = shapes
        .into_iter()
        .map(|shape| shape.chars().filter(|&c| c == '#').count())
        .collect();

    Some(
        regions
            .lines()
            .flat_map(|line| {
                let (dim, counts) = line.split_once(": ")?;
                let (w, h) = scanf!(dim, "{}x{}", usize, usize).ok()?;
                let area = counts
                    .split_whitespace()
                    .enumerate()
                    .filter_map(|(i, n)| {
                        let n = n.parse::<usize>().ok()?;
                        Some(n * shapes[i])
                    })
                    .sum::<usize>();
                Some(area <= w * h)
            })
            .filter(|ok| *ok)
            .count() as u64,
    )
}

pub fn part_two(_input: &str) -> Option<u64> {
    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::read_file;

    #[test]
    fn test_part_one() {
        #[cfg(not(feature = "ci"))]
        assert_eq!(part_one(&read_file("inputs", DAY)), Some(422));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&read_file("examples", DAY)), Some(0));
    }
}
