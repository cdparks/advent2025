use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use advent_of_code::point::Point3;
use sscanf::scanf;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse(input);

    // example only uses 10 pairs for some reason
    let wanted = if points.len() < 1000 { 10 } else { 1000 };

    let mut pairs: Vec<(Point, Point)> = points
        .iter()
        .enumerate()
        .flat_map(|(i, p)| points.iter().skip(i + 1).map(|q| (*p, *q)))
        .collect();

    pairs.sort_by_key(|(p, q)| p.dist_squared(*q));

    // Use reference-counted circuits to avoid a lot of cloning
    let mut circuits: HashMap<Point, Rc<HashSet<Point>>> = points
        .into_iter()
        .map(|p| (p, Rc::new(HashSet::from_iter(vec![p]))))
        .collect();

    for (p, q) in pairs.into_iter().take(wanted) {
        let merged: Rc<HashSet<Point>> =
            Rc::new(circuits[&p].union(&circuits[&q]).copied().collect());
        for u in merged.iter() {
            circuits.insert(*u, merged.clone());
        }
    }

    let mut sizes: Vec<usize> = circuits
        .into_values()
        .flat_map(|circuit| Rc::into_inner(circuit)) // unique via refcount
        .map(|circuit| circuit.len())
        .collect();

    sizes.sort();

    Some(sizes.into_iter().rev().take(3).product::<usize>() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse(input);
    let wanted = points.len();

    let mut pairs: Vec<(Point, Point)> = points
        .iter()
        .enumerate()
        .flat_map(|(i, p)| points.iter().skip(i + 1).map(|q| (*p, *q)))
        .collect();

    pairs.sort_by_key(|(p, q)| p.dist_squared(*q));

    let mut circuits: HashMap<Point, Rc<HashSet<Point>>> = points
        .into_iter()
        .map(|p| (p, Rc::new(HashSet::from_iter(vec![p]))))
        .collect();

    for (p, q) in pairs.into_iter() {
        let merged: Rc<HashSet<Point>> =
            Rc::new(circuits[&p].union(&circuits[&q]).copied().collect());
        for u in merged.iter() {
            circuits.insert(*u, merged.clone());
        }
        if merged.len() >= wanted {
            return Some((p.x * q.x) as u64);
        }
    }
    None
}

type Point = Point3<i64>;

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .flat_map(|line| {
            let (x, y, z) = scanf!(line, "{},{},{}", i64, i64, i64).ok()?;
            Some(Point3::new(x, y, z))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::read_file;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&read_file("examples", DAY)), Some(40));
        #[cfg(not(feature = "ci"))]
        assert_eq!(part_one(&read_file("inputs", DAY)), Some(46398));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&read_file("examples", DAY)), Some(25272));
        #[cfg(not(feature = "ci"))]
        assert_eq!(part_two(&read_file("inputs", DAY)), Some(8141888143));
    }
}
