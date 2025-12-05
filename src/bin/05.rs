use sscanf::sscanf;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (mut ranges, ids) = parse(input)?;
    merge_intervals(&mut ranges);
    Some(
        ids.into_iter()
            .filter(|id| ranges.iter().any(|(lo, hi)| lo <= id && id <= hi))
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut ranges, _) = parse(input)?;
    merge_intervals(&mut ranges);
    Some(ranges.into_iter().map(|(lo, hi)| 1 + hi - lo).sum::<usize>() as u64)
}

fn merge_intervals(ranges: &mut Vec<Interval>) {
    if ranges.is_empty() {
        return;
    }

    ranges.sort();

    let mut i = 0;
    for j in 1..ranges.len() {
        if let Some(merged) = try_merge(ranges[i], ranges[j]) {
            ranges[i] = merged;
        } else {
            i += 1;
            ranges[i] = ranges[j];
        }
    }
    ranges.truncate(i + 1);
}

fn try_merge((lo1, hi1): Interval, (lo2, hi2): Interval) -> Option<Interval> {
    if hi1 >= lo2 {
        return Some((lo1, hi1.max(hi2)));
    }
    None
}

fn parse(input: &str) -> Option<(Vec<Interval>, Vec<usize>)> {
    let (ranges, ids) = input.split_once("\n\n")?;
    let ranges: Vec<_> = ranges
        .lines()
        .flat_map(|line| sscanf!(line, "{}-{}", usize, usize))
        .collect();
    let ids = ids.lines().flat_map(|line| line.parse()).collect();
    Some((ranges, ids))
}

type Interval = (usize, usize);

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::read_file;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&read_file("examples", DAY)), Some(3));
        #[cfg(not(feature = "ci"))]
        assert_eq!(part_one(&read_file("inputs", DAY)), Some(690));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&read_file("examples", DAY)), Some(14));
        #[cfg(not(feature = "ci"))]
        assert_eq!(part_two(&read_file("inputs", DAY)), Some(344323629240733));
    }
}
