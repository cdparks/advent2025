use advent_of_code::point::Point3;
use disjoint::DisjointSet;
use itertools::Itertools;
use sscanf::scanf;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse(input);

    // example only uses 10 pairs for some reason
    let wanted = if points.len() < 1000 { 10 } else { 1000 };
    let pairs: Vec<(usize, usize)> = (0..points.len())
        .cartesian_product(0..points.len())
        .filter(|(i, j)| i < j)
        .sorted_by_key(|(i, j)| points[*i].dist_squared(points[*j]))
        .take(wanted)
        .collect();

    let mut circuits = DisjointSet::with_len(points.len());

    for (i, j) in pairs.into_iter() {
        if !circuits.is_joined(i, j) {
            circuits.join(i, j);
        }
    }

    Some(
        circuits
            .sets()
            .into_iter()
            .map(|circuit| circuit.len())
            .k_largest(3)
            .product::<usize>() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse(input);
    let pairs: Vec<(usize, usize)> = (0..points.len())
        .cartesian_product(0..points.len())
        .filter(|(i, j)| i < j)
        .sorted_by_key(|(i, j)| points[*i].dist_squared(points[*j]))
        .collect();

    let mut circuits = DisjointSet::with_len(points.len());
    let mut num_circuits = points.len();

    for (i, j) in pairs.into_iter() {
        if !circuits.is_joined(i, j) {
            circuits.join(i, j);
            num_circuits -= 1;
            if num_circuits <= 1 {
                return Some((points[i].x * points[j].x) as u64);
            }
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
