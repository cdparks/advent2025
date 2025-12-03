use sscanf::sscanf;
use std::iter::repeat_n;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        parse_rotations(input)
            .iter()
            .scan(50, |acc, rot| {
                *acc = (*acc + rot).rem_euclid(100);
                Some(*acc)
            })
            .filter(|x| *x == 0)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(
        parse_rotations(input)
            .iter()
            .flat_map(|n| {
                let d = if *n < 0 { -1i32 } else { 1i32 };
                repeat_n(d, n.unsigned_abs() as usize)
            })
            .fold((0, 50), |(zeroes, pos), d| {
                let pos = (pos + d).rem_euclid(100);
                (zeroes + if pos == 0 { 1 } else { 0 }, pos)
            })
            .0,
    )
}

fn parse_rotations(s: &str) -> Vec<i32> {
    s.lines()
        .filter_map(|line| {
            let (dir, n) = sscanf!(line, "{}{}", char, i32).ok()?;
            Some(if dir == 'L' { -n } else { n })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&advent_of_code::template::read_file("examples", DAY)),
            Some(3)
        );
        assert_eq!(
            part_one(&advent_of_code::template::read_file("inputs", DAY)),
            Some(1195)
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&advent_of_code::template::read_file("examples", DAY)),
            Some(6)
        );
        assert_eq!(
            part_two(&advent_of_code::template::read_file("inputs", DAY)),
            Some(6770)
        );
    }
}
