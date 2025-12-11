use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let graph = parse(input);
    Some(count_paths(&graph, "you", true, true, "out") as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph = parse(input);
    Some(count_paths(&graph, "svr", false, false, "out") as u64)
}

fn count_paths<'input>(graph: &Graph<'input>, src: &str, fft: bool, dac: bool, dst: &str) -> usize {
    return dfs(&mut HashMap::new(), graph, src, fft, dac, dst);

    fn dfs<'node, 'input: 'node>(
        memo: &mut Memo<'node>,
        graph: &Graph<'input>,
        node: &'node str,
        fft: bool,
        dac: bool,
        dst: &str,
    ) -> usize {
        if node == dst {
            return if fft && dac { 1 } else { 0 };
        }

        let key = (node, fft, dac);
        if let Some(&count) = memo.get(&key) {
            return count;
        }

        let Some(neighbors) = graph.get(node) else {
            return 0;
        };

        let count = neighbors
            .iter()
            .map(|neighbor| {
                dfs(
                    memo,
                    graph,
                    neighbor,
                    fft || neighbor == &"fft",
                    dac || neighbor == &"dac",
                    dst,
                )
            })
            .sum();

        memo.insert(key, count);
        count
    }
}

fn parse(input: &str) -> Graph<'_> {
    input
        .lines()
        .flat_map(|line| {
            let (node, rest) = line.split_once(": ")?;
            Some((node, rest.split_whitespace().collect()))
        })
        .collect()
}

type Memo<'node> = HashMap<(&'node str, bool, bool), usize>;
type Graph<'input> = HashMap<&'input str, Vec<&'input str>>;

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::{read_file, read_file_part};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&read_file("examples", DAY)), Some(5));
        #[cfg(not(feature = "ci"))]
        assert_eq!(part_one(&read_file("inputs", DAY)), Some(590));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&read_file_part("examples", DAY, 2)), Some(2));
        #[cfg(not(feature = "ci"))]
        assert_eq!(part_two(&read_file("inputs", DAY)), Some(319473830844560));
    }
}
