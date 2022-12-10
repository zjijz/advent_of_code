use anyhow::Context;
use itertools::Itertools as _;
use regex::Regex;
use std::{collections::VecDeque, path::Path, str::FromStr};

#[derive(Debug, Clone, Copy, Default)]
struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

macro regex_capture_group_to_usize($captures:expr, $group:expr) {
    $captures
        .get($group)
        .map(|m| m.as_str())
        .ok_or_else(|| anyhow::anyhow!("Could not find match group 1."))
        .and_then(|s| {
            usize::from_str(s).map_err(|_| anyhow::anyhow!("Could not parse {} into usize.", s))
        })
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Lazy static for regex.
        let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)")
            .with_context(|| "Could not compile the regex!")?;

        let captures = regex
            .captures(s)
            .ok_or_else(|| anyhow::anyhow!("No match was found in {}!", s))?;
        let quantity = regex_capture_group_to_usize!(captures, 1)?;
        let from = regex_capture_group_to_usize!(captures, 2)?;
        let to = regex_capture_group_to_usize!(captures, 3)?;
        Ok(Move { quantity, from, to })
    }
}

#[derive(Debug, Clone)]
struct Stacks {
    stacks: Vec<VecDeque<char>>,
}

impl FromStr for Stacks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stacks_len = (s.find("\n").ok_or(anyhow::anyhow!("Input chart was not formatted with 3-space crates with additional one space separators."))? + 1) / 4;
        let mut stacks = vec![VecDeque::new(); stacks_len];
        for line in s.lines().rev().skip(1) {
            for (i, item) in line.bytes().into_iter().chunks(4).into_iter().enumerate() {
                let vec = item.into_iter().collect::<Vec<_>>();
                let s = std::str::from_utf8(&vec).with_context(|| {
                    "Chunk was parsed incorrectly in a way unable to recreate a str."
                })?;
                if s.trim() == "" {
                    continue;
                }
                let indicator = s.chars().nth(1).ok_or_else(|| {
                    anyhow::anyhow!("Could not retreive the crate letter from the item.")
                })?;
                stacks[i].push_back(indicator);
            }
        }
        Ok(Self { stacks })
    }
}

fn get_stacks_as_mut_ref<'a>(
    stacks: &'a mut Stacks,
    from: usize,
    to: usize,
) -> (&'a mut VecDeque<char>, &'a mut VecDeque<char>) {
    let min = std::cmp::min(from, to);
    let (left, right) = stacks.stacks.split_at_mut(min + 1);
    if min == from {
        (&mut left[left.len() - 1], &mut right[to - from - 1])
    } else {
        // min == to
        (&mut right[from - to - 1], &mut left[left.len() - 1])
    }
}

impl Stacks {
    fn simulate<const PART: usize>(&mut self, m: Move) -> anyhow::Result<()> {
        if m.quantity == 0 || m.from == m.to {
            return Ok(());
        }
        let (from, to) = get_stacks_as_mut_ref(self, m.from - 1, m.to - 1);
        anyhow::ensure!(
            from.len() >= m.quantity,
            "From stack is not big enough to accomodate moving {} crates!",
            m.quantity,
        );
        let copy = from.split_off(from.len() - m.quantity);
        if PART == 1 {
            to.extend(copy.into_iter().rev());
        } else {
            // PART == 2
            to.extend(copy);
        }
        Ok(())
    }

    fn tops(&self) -> anyhow::Result<String> {
        let mut s = String::with_capacity(self.stacks.len());
        for (i, stack) in self.stacks.iter().enumerate() {
            s.push(
                *stack
                    .back()
                    .ok_or_else(|| anyhow::anyhow!("Stack {} was empty!", i))?,
            );
        }
        Ok(s)
    }
}

pub async fn part<const PART: usize>(path: impl AsRef<Path>) -> anyhow::Result<String> {
    let contents = tokio::fs::read_to_string(path).await?;

    // This really shouldn't leak. I thought I would be able to unleak it at the
    // bottom of the function, but it complains about the &mut str being
    // borrowed too long (for 'static).
    let contents = contents.leak();

    let (mut stacks, moves): (_, Vec<_>) =
        match contents.split_at(contents.find("\n\n").unwrap_or(contents.len())) {
            (stacks, moves) => {
                let stacks = Stacks::from_str(stacks)?;
                let moves = futures::future::join_all(
                    moves
                        .trim()
                        .lines()
                        .map(|line| tokio::spawn(async move { Move::from_str(line) })),
                )
                .await
                .into_iter()
                .collect::<Result<_, _>>()?;
                (stacks, moves)
            }
        };

    for maybe_move in moves.into_iter() {
        if let Err(err) = maybe_move {
            anyhow::bail!(err);
        }

        let m = maybe_move.unwrap();
        stacks.simulate::<PART>(m)?;
    }
    stacks.tops()
}
