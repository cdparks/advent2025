advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines: Vec<&str> = input.lines().collect();
    let last = lines.pop()?;

    let mut grid: Vec<_> = lines
        .into_iter()
        .map(|line| line.split_whitespace().flat_map(|n| n.parse::<u64>().ok()))
        .collect();

    Some(
        last.split_whitespace()
            .flat_map(|s| {
                let col = grid.iter_mut().filter_map(|col| col.next());
                Some(match s.parse().ok()? {
                    Op::Add => col.sum::<u64>(),
                    Op::Mul => col.product(),
                })
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines: Vec<&str> = input.lines().collect();
    let mut iter = lines.pop()?.chars().enumerate().peekable();

    let mut ops: Vec<(Op, usize, usize)> = vec![];
    while let Some((col, op @ ('+' | '*'))) = iter.next() {
        let mut span = 1;
        while let Some((_, c)) = iter.peek() {
            if c.is_whitespace() {
                iter.next();
                span += 1;
            } else {
                span -= 1; // scooch back, we hit the next one
                break;
            }
        }
        ops.push((op.to_string().parse().ok()?, col, span));
    }

    let mut grid: Vec<Vec<u64>> = ops.iter().map(|(_, _, span)| vec![0; *span]).collect();

    for line in lines.into_iter() {
        let chars: Vec<char> = line.chars().collect();
        for (i, (_, col, span)) in ops.iter().copied().enumerate() {
            for j in 0..span {
                if chars[col + j].is_digit(10) {
                    grid[i][j] = grid[i][j] * 10 + chars[col + j] as u64 - '0' as u64;
                }
            }
        }
    }

    Some(
        ops.into_iter()
            .enumerate()
            .map(|(i, (op, _, _))| match op {
                Op::Add => grid[i].iter().sum::<u64>(),
                Op::Mul => grid[i].iter().product(),
            })
            .sum(),
    )
}

#[derive(Copy, Clone)]
enum Op {
    Add,
    Mul,
}

impl std::str::FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Mul),
            _ => Err(format!("unrecognized Op: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::read_file;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&read_file("examples", DAY)), Some(4277556));
        #[cfg(not(feature = "ci"))]
        assert_eq!(part_one(&read_file("inputs", DAY)), Some(3785892992137));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&read_file("examples", DAY)), Some(3263827));
        #[cfg(not(feature = "ci"))]
        assert_eq!(part_two(&read_file("inputs", DAY)), Some(7669802156452))
    }
}
