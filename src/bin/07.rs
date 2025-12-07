use advent_of_code::point::Point2;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(&parse(&input)?).splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(&parse(&input)?).paths)
}

fn solve(grid: &Grid) -> Answer {
    fn go(splits: &mut u64, memo: &mut HashMap<Point, u64>, grid: &Grid, p: Point) -> u64 {
        if let Some(paths) = memo.get(&p) {
            return *paths;
        }

        let paths = if grid.splitters.contains(&p) {
            *splits += 1;
            go(splits, memo, grid, p.left()) + go(splits, memo, grid, p.right())
        } else if p.y < grid.height {
            go(splits, memo, grid, p.down())
        } else {
            1
        };
        memo.insert(p, paths);
        paths
    }

    let mut splits = 0;
    let mut memo = HashMap::new();
    let paths = go(&mut splits, &mut memo, grid, grid.start);
    Answer { splits, paths }
}

fn parse(input: &str) -> Option<Grid> {
    let mut height = 0;
    let start = Point::new(input.lines().next()?.find('S')? as i64, 0);
    let splitters = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            height += 1;
            line.chars().enumerate().flat_map(move |(x, c)| match c {
                '^' => Some(Point::new(x as i64, y as i64)),
                _ => None,
            })
        })
        .collect();
    Some(Grid {
        start,
        height,
        splitters,
    })
}

type Point = Point2<i64>;

struct Grid {
    start: Point,
    height: i64,
    splitters: HashSet<Point>,
}

struct Answer {
    splits: u64,
    paths: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::read_file;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&read_file("examples", DAY)), Some(21));
        #[cfg(not(feature = "ci"))]
        assert_eq!(part_one(&read_file("inputs", DAY)), Some(1626));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&read_file("examples", DAY)), Some(40));
        #[cfg(not(feature = "ci"))]
        assert_eq!(part_two(&read_file("inputs", DAY)), Some(48989920237096));
    }
}
