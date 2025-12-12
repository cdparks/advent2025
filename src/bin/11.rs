use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let graph = parse(input);
    Some(count_paths(&graph, "you", [], "out") as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph = parse(input);
    Some(count_paths(&graph, "svr", ["fft", "dac"], "out") as u64)
}

fn count_paths<'input, const N: usize>(
    graph: &Graph<'input>,
    src: &str,
    waypoints: [&str; N],
    dst: &str,
) -> usize {
    return dfs(
        &mut HashMap::new(),
        graph,
        src,
        waypoints.map(|w| (w, false)),
        dst,
    );

    fn dfs<'node, 'input: 'node, 'waypoint, const N: usize>(
        memo: &mut Memo<'node, 'waypoint, N>,
        graph: &Graph<'input>,
        node: &'node str,
        waypoints: [(&'waypoint str, bool); N],
        dst: &str,
    ) -> usize {
        if node == dst {
            return if waypoints.iter().all(|(_, seen)| *seen) {
                1
            } else {
                0
            };
        }

        let key = (node, waypoints);
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
                    waypoints.map(|(w, seen)| (w, seen || w == node)),
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

type Memo<'node, 'waypoint, const N: usize> = HashMap<(&'node str, [(&'waypoint str, bool); N]), usize>;
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
