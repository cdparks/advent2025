use std::collections::HashMap;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 12))
}

fn solve(input: &str, width: usize) -> u64 {
    input
        .lines()
        .flat_map(|line| {
            let digits: Vec<u64> = line.chars().map(|c| c as u64 - '0' as u64).collect();
            solve_memo(&mut HashMap::new(), &digits, 0, width)
        })
        .sum()
}

fn solve_memo(
    memo: &mut HashMap<(usize, usize), Option<u64>>,
    digits: &[u64],
    i: usize,
    remaining: usize,
) -> Option<u64> {
    (i..digits.len())
        .into_iter()
        .flat_map(|j| {
            let key = (j, remaining);
            match memo.get(&key) {
                None => {
                    let value = if remaining <= 1 {
                        Some(digits[j])
                    } else if remaining <= digits.len() - j {
                        let d = digits[j] * 10u64.pow(remaining as u32 - 1);
                        solve_memo(memo, digits, j + 1, remaining - 1).map(|m| d + m)
                    } else {
                        None
                    };
                    memo.insert(key, value);
                    value
                }
                Some(m) => *m,
            }
        })
        .max()
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
