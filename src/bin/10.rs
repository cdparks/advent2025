use good_lp::*;
use itertools::Itertools;
advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .into_iter()
            .map(|(goal, banks, _)| {
                for set in banks.into_iter().powerset() {
                    let presses = set.len() as u64;
                    if set.into_iter().fold(0, |acc, b| acc ^ b) == goal {
                        return presses;
                    }
                }
                0
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .into_iter()
            .map(|(_, banks, joltage)| {
                let mut vars = variables!();
                let presses: Vec<Variable> = banks
                    .iter()
                    .map(|_| vars.add(variable().min(0).integer()))
                    .collect();

                let width = joltage.len() - 1;
                let solution = vars
                    .minimise(presses.iter().sum::<Expression>())
                    .using(default_solver)
                    .with_all(joltage.iter().enumerate().map(|(i, jolt)| {
                        let sum = banks
                            .iter()
                            .zip(presses.iter())
                            .flat_map(|(bank, press)| {
                                if ((1 << (width - i)) & bank) != 0 {
                                    Some(*press)
                                } else {
                                    None
                                }
                            })
                            .sum::<Expression>();
                        constraint!(sum == *jolt)
                    }))
                    .solve()
                    .unwrap();

                presses.into_iter().map(|p| solution.value(p)).sum::<f64>() as u64
            })
            .sum::<u64>(),
    )
}

fn parse(input: &str) -> Vec<(u16, Vec<u16>, Vec<u16>)> {
    input.lines().flat_map(|line| parse_line(line)).collect()
}

// Represent button banks as bitmasks
//
// [.##.] (___3) (_1_3) (__2_) (__23) (0_2_) (01__) {3,5,4,7}
//  0110   ___1   _101   __10   __11   1010   1100
fn parse_line(line: &str) -> Option<(u16, Vec<u16>, Vec<u16>)> {
    let (indicators, rest) = line[1..].split_once("] ")?;
    let (banks, rest) = rest.split_once(" {")?;

    let goal: u16 = indicators.chars().fold(0, |acc, c| {
        let bit = if c == '#' { 1 } else { 0 };
        2 * acc + bit
    });

    let width = indicators.len() - 1;
    let banks: Vec<u16> = banks
        .split_whitespace()
        .map(|bank| {
            let bank = &bank[1..bank.len() - 1];
            bank.split(",")
                .flat_map(|n| n.parse::<u32>().ok())
                .fold(0, |acc, n| acc + 2u16.pow(width as u32 - n))
        })
        .collect();

    let joltage: Vec<u16> = rest[..rest.len() - 1]
        .split(",")
        .flat_map(|n| n.parse::<u16>().ok())
        .collect();

    Some((goal, banks, joltage))
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::read_file;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&read_file("examples", DAY)), Some(7));
        #[cfg(not(feature = "ci"))]
        assert_eq!(part_one(&read_file("inputs", DAY)), Some(401));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&read_file("examples", DAY)), Some(33));
        #[cfg(not(feature = "ci"))]
        assert_eq!(part_two(&read_file("inputs", DAY)), Some(15017));
    }
}
