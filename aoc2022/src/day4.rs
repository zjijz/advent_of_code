use anyhow::Context as _;
use rayon::iter::ParallelIterator as _;
use rayon::str::ParallelString as _;
use std::collections::HashSet;
use std::path::Path;

fn str_to_range(s: impl AsRef<str>) -> anyhow::Result<std::ops::RangeInclusive<usize>> {
    let items = s.as_ref().split("-").collect::<Vec<_>>();
    anyhow::ensure!(items.len() == 2, "The split term must have two pieces");
    let left: usize = items[0]
        .parse()
        .with_context(|| format!("{} could not be parsed into a usize.", items[0]))?;
    let right: usize = items[1]
        .parse()
        .with_context(|| format!("{} could not be parsed into a usize.", items[1]))?;
    Ok(left..=right)
}

pub fn part1(path: impl AsRef<Path>) -> anyhow::Result<usize> {
    let contents = std::fs::read_to_string(path)?;
    contents
        .par_lines()
        .map(|line| {
            let items = line.split(",").collect::<Vec<_>>();
            anyhow::ensure!(
                items.len() == 2,
                "The split line should only have two pieces."
            );

            let first = str_to_range(items[0])?.collect::<HashSet<_>>();
            let second = str_to_range(items[1])?.collect::<HashSet<_>>();

            let (smaller, larger) = if first.len() > second.len() {
                (&second, &first)
            } else {
                (&first, &second)
            };
            Ok(if larger.is_superset(smaller) { 1 } else { 0 })
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
        .par_lines()
        .map(|line| {
            let items = line.split(",").collect::<Vec<_>>();
            anyhow::ensure!(
                items.len() == 2,
                "The split line should only have two pieces."
            );

            let first = str_to_range(items[0])?.collect::<HashSet<_>>();
            let second = str_to_range(items[1])?.collect::<HashSet<_>>();

            Ok(if first.is_disjoint(&second) { 0 } else { 1 })
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
