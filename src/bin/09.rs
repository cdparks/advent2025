use advent_of_code::point::Point2;

use itertools::Itertools;
use sscanf::scanf;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let tiles = parse(input);

    tiles
        .iter()
        .enumerate()
        .cartesian_product(tiles.iter().enumerate())
        .flat_map(
            |((i, p), (j, q))| {
                if i < j { Some(area_rect(p, q)) } else { None }
            },
        )
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut tiles = parse(input);
    tiles.push(tiles[0]); // wrap around

    tiles
        .iter()
        .enumerate()
        .cartesian_product(tiles.iter().enumerate())
        .flat_map(|((i, p), (j, q))| {
            if i < j && is_inside(&tiles, &Rect::new(*p, *q)) {
                Some(area_rect(p, q))
            } else {
                None
            }
        })
        .max()
}

fn area_rect(p: &Point, q: &Point) -> u64 {
    let dx = (p.x - q.x).abs() + 1;
    let dy = (p.y - q.y).abs() + 1;
    (dx * dy) as u64
}

fn is_inside(polygon: &Vec<Point>, rect: &Rect) -> bool {
    polygon
        .iter()
        .zip(polygon.iter().skip(1))
        .filter_map(|(p, q)| Segment::new(*p, *q))
        .all(|line| !line.intersects(rect))
}

struct Rect {
    xmin: i64,
    ymin: i64,
    xmax: i64,
    ymax: i64,
}

impl Rect {
    fn new(p: Point, q: Point) -> Self {
        let xmin = p.x.min(q.x);
        let ymin = p.y.min(q.y);
        let xmax = p.x.max(q.x);
        let ymax = p.y.max(q.y);
        Rect {
            xmin,
            ymin,
            xmax,
            ymax,
        }
    }
}

enum Segment {
    Horizontal { y: i64, xmin: i64, xmax: i64 },
    Vertical { x: i64, ymin: i64, ymax: i64 },
}

impl Segment {
    fn new(p: Point, q: Point) -> Option<Segment> {
        if p.y == q.y {
            Some(Segment::Horizontal {
                y: p.y,
                xmin: p.x.min(q.x),
                xmax: p.x.max(q.x),
            })
        } else if p.x == q.x {
            Some(Segment::Vertical {
                x: p.x,
                ymin: p.y.min(q.y),
                ymax: p.y.max(q.y),
            })
        } else {
            None
        }
    }

    #[rustfmt::skip]
    fn intersects(self, rect: &Rect) -> bool {
        match self {
            Segment::Horizontal { y, xmin, xmax } => {
                rect.ymin < y && y < rect.ymax
                    && (xmin <= rect.xmin && rect.xmin < xmax
                        || xmin < rect.xmax && rect.xmax <= xmax)
            }
            Segment::Vertical { x, ymin, ymax } => {
                rect.xmin < x && x < rect.xmax
                    && (ymin <= rect.ymin && rect.ymin < ymax
                        || ymin < rect.ymax && rect.ymax <= ymax)
            }
        }
    }
}

type Point = Point2<i64>;

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .flat_map(|line| {
            let (x, y) = scanf!(line, "{},{}", i64, i64).ok()?;
            Some(Point2::new(x, y))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::read_file;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&read_file("examples", DAY)), Some(50));
        #[cfg(not(feature = "ci"))]
        assert_eq!(part_one(&read_file("inputs", DAY)), Some(4748769124));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&read_file("examples", DAY)), Some(24));
        #[cfg(not(feature = "ci"))]
        assert_eq!(part_two(&read_file("inputs", DAY)), Some(1525991432));
    }
}
