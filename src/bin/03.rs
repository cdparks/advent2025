advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 12))
}

fn solve(input: &str, width: usize) -> u64 {
    fn go(digits: &[u64], acc: u64, n: usize) -> Option<u64> {
        if n == 0 {
            Some(acc)
        } else {
            digits[..=digits.len() - n]
                .iter()
                .enumerate()
                // make max_by_key return the first max in case of ties
                .rev()
                .max_by_key(|(_, d)| *d)
                .and_then(|(i, d)| go(&digits[i + 1..], 10 * acc + d, n - 1))
        }
    }

    input
        .lines()
        .flat_map(|line| {
            let digits: Vec<u64> = line.chars().map(|c| c as u64 - '0' as u64).collect();
            go(&digits, 0, width)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&advent_of_code::template::read_file("examples", DAY)),
            Some(357)
        );
        assert_eq!(
            part_one(&advent_of_code::template::read_file("inputs", DAY)),
            Some(17554)
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&advent_of_code::template::read_file("examples", DAY)),
            Some(3121910778619)
        );
        assert_eq!(
            part_two(&advent_of_code::template::read_file("inputs", DAY)),
            Some(175053592950232)
        );
    }
}
