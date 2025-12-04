use advent_of_code::point::Point2;
use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    Some(accessible_rolls(&parse_rolls(input)).count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(remove_accessible_rolls(&mut parse_rolls(input)))
}

type Point = Point2<isize>;

fn remove_accessible_rolls(rolls: &mut HashSet<Point>) -> u64 {
    let mut count = 0;
    let mut queue: VecDeque<_> = accessible_rolls(rolls).collect();
    while let Some(p) = queue.pop_front() {
        if !rolls.contains(&p) {
            continue;
        }
        if is_accessible(rolls, p) {
            count += 1;
            rolls.remove(&p);
            queue.extend(neighbors(rolls, p));
        }
    }
    count
}

fn accessible_rolls(rolls: &HashSet<Point>) -> impl Iterator<Item = Point> {
    rolls
        .iter()
        .copied()
        .filter(move |p| is_accessible(rolls, *p))
}

fn is_accessible(rolls: &HashSet<Point>, p: Point) -> bool {
    neighbors(&rolls, p).count() < 4
}

fn neighbors(rolls: &HashSet<Point>, p: Point) -> impl Iterator<Item = Point> {
    p.neighbors8().into_iter().filter(|q| rolls.contains(&q))
}

fn parse_rolls(input: &str) -> HashSet<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '@' {
                    Some(Point2::new(x as isize, y as isize))
                } else {
                    None
                }
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::read_file;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&read_file("examples", DAY)), Some(13));
        #[cfg(not(feature = "ci"))]
        assert_eq!(part_one(&read_file("inputs", DAY)), Some(1349));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&read_file("examples", DAY)), Some(43));
        #[cfg(not(feature = "ci"))]
        assert_eq!(part_two(&read_file("inputs", DAY)), Some(8277));
    }
}
