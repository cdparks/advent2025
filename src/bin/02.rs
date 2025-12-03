use sscanf::sscanf;
use rayon::prelude::*;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    sum_invalid(input, |x| {
        let s = x.to_string();
        if !s.len().is_multiple_of(2) {
            return false;
        }
        let (rhs, lhs) = s.split_at(s.len() / 2);
        lhs == rhs
    })
}

pub fn part_two(input: &str) -> Option<u64> {
    sum_invalid(input, |x| {
        let chars: Vec<char> = x.to_string().chars().collect();
        for i in 1..=chars.len() / 2 {
            if !chars.len().is_multiple_of(i) {
                continue;
            }
            let mut iter = chars.chunks_exact(i);
            let first = iter.next().unwrap();
            if iter.all(|chunk| chunk == first) {
                return true;
            }
        }
        false
    })
}

fn sum_invalid<P>(input: &str, p: P) -> Option<u64>
where
    P: Send + Sync + Fn(&u64) -> bool,
{
    Some(
        input
            .trim()
            .par_split(',')
            .filter_map(|range| {
                let (lo, hi) = sscanf!(range, "{}-{}", u64, u64).ok()?;
                Some((lo, hi))
            })
            .flat_map(|(lo, hi)| lo..=hi)
            .filter(p)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&advent_of_code::template::read_file("examples", DAY)),
            Some(1227775554)
        );
        assert_eq!(
            part_one(&advent_of_code::template::read_file("inputs", DAY)),
            Some(18700015741)
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&advent_of_code::template::read_file("examples", DAY)),
            Some(4174379265)
        );
        assert_eq!(
            part_two(&advent_of_code::template::read_file("inputs", DAY)),
            Some(20077272987)
        );
    }
}
