use rayon::iter::ParallelBridge as _;
use rayon::iter::ParallelIterator as _;
use rayon::str::ParallelString as _;
use std::collections::HashSet;
use std::path::Path;

pub fn part1(path: impl AsRef<Path>) -> anyhow::Result<usize> {
    let contents = std::fs::read_to_string(path)?;
    contents
        .par_lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);

            // TODO: Change this to create HashSet for left, then iterate over right until a single match is found.
            let left = left.chars().collect::<HashSet<_>>();
            let right = right.chars().collect::<HashSet<_>>();
            Ok(left
                .intersection(&right)
                .map(|item| {
                    if item.is_lowercase() {
                        *item as usize - 96
                    } else {
                        *item as usize - 38
                    }
                })
                .sum())
        })
        .try_reduce(
            || 0usize,
            |a, b| {
                a.checked_add(b).ok_or_else(|| {
                    anyhow::anyhow!("Could not add {} and {} without overflow!", a, b)
                })
            },
        )
}

pub fn part2(path: impl AsRef<Path>) -> anyhow::Result<usize> {
    let contents = std::fs::read_to_string(path)?;
    contents
        .lines()
        .array_chunks::<3>()
        .par_bridge()
        .map(|lines| {
            // TODO: Find all common between first and second HashSets.
            // TODO: Then manually find first overlap between that and third.
            let first = lines[0].chars().collect::<HashSet<_>>();
            let second = lines[1].chars().collect::<HashSet<_>>();
            let third = lines[2].chars().collect::<HashSet<_>>();
            let common = {
                let common = first.intersection(&second).copied().collect::<HashSet<_>>();
                common.intersection(&third).copied().collect::<Vec<_>>()
            };
            anyhow::ensure!(common.len() == 1, "");
            Ok({
                let badge = common[0];
                if badge.is_lowercase() {
                    badge as usize - 96
                } else {
                    badge as usize - 38
                }
            })
        })
        .try_reduce(
            || 0usize,
            |a, b| {
                a.checked_add(b).ok_or_else(|| {
                    anyhow::anyhow!("Could not add {} and {} without overflow!", a, b)
                })
            },
        )
}
