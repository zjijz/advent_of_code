use rayon::iter::ParallelIterator as _;
use rayon::str::ParallelString as _;
use std::path::Path;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum Outcome {
    Lost = 0,
    Draw = 3,
    Won = 6,
}

impl Outcome {
    fn from_symbol(input: &str) -> anyhow::Result<Outcome> {
        match input {
            "X" => Ok(Outcome::Lost),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Won),
            _ => Err(anyhow::anyhow!(format!(
                "{} is not one of the supported values (\"X\", \"Y\", \"Z\").",
                input,
            ))),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Play {
    fn from_opponent(input: &str) -> anyhow::Result<Self> {
        match input {
            "A" => Ok(Play::Rock),
            "B" => Ok(Play::Paper),
            "C" => Ok(Play::Scissors),
            _ => Err(anyhow::anyhow!(format!(
                "{} is not one of the supported values (\"A\", \"B\", \"C\").",
                input,
            ))),
        }
    }

    fn from_ipseity(input: &str) -> anyhow::Result<Self> {
        match input {
            "X" => Ok(Play::Rock),
            "Y" => Ok(Play::Paper),
            "Z" => Ok(Play::Scissors),
            _ => Err(anyhow::anyhow!(format!(
                "{} is not one of the supported values (\"X\", \"Y\", \"Z\")",
                input,
            ))),
        }
    }

    fn outcome(opponent: Play, ipseity: Play) -> Outcome {
        match (opponent, ipseity) {
            (Play::Rock, Play::Scissors)
            | (Play::Paper, Play::Rock)
            | (Play::Scissors, Play::Paper) => Outcome::Lost,
            (a, b) if a == b => Outcome::Draw,
            _ => Outcome::Won,
        }
    }

    fn trumps(&self) -> Play {
        match self {
            Play::Rock => Play::Scissors,
            Play::Paper => Play::Rock,
            Play::Scissors => Play::Paper,
        }
    }

    fn loses_to(&self) -> Play {
        match self {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock,
        }
    }
}

fn calculate_part1(opponent: Play, ipseity: Play) -> usize {
    ipseity as usize + Play::outcome(opponent, ipseity) as usize
}

pub fn part1(path: impl AsRef<Path>) -> anyhow::Result<usize> {
    let contents = std::fs::read_to_string(path)?;
    contents
        .par_lines()
        .map(|line| -> anyhow::Result<usize> {
            match line.split(' ').collect::<Vec<_>>().as_slice() {
                [opponent, ipseity] => {
                    let opponent = Play::from_opponent(*opponent)?;
                    let ipseity = Play::from_ipseity(*ipseity)?;
                    Ok(calculate_part1(opponent, ipseity))
                }
                _ => Err(anyhow::anyhow!(format!(
                    "{} had more than two items!",
                    line
                ))),
            }
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

fn calculate_part2(opponent: Play, outcome: Outcome) -> usize {
    outcome as usize
        + match outcome {
            Outcome::Lost => opponent.trumps() as usize,
            Outcome::Draw => opponent as usize,
            Outcome::Won => opponent.loses_to() as usize,
        }
}

pub fn part2(path: impl AsRef<Path>) -> anyhow::Result<usize> {
    let contents = std::fs::read_to_string(path)?;
    contents
        .par_lines()
        .map(|line| -> anyhow::Result<usize> {
            match line.split(' ').collect::<Vec<_>>().as_slice() {
                [opponent, outcome] => {
                    let opponent = Play::from_opponent(*opponent)?;
                    let outcome = Outcome::from_symbol(*outcome)?;
                    Ok(calculate_part2(opponent, outcome))
                }
                _ => Err(anyhow::anyhow!(format!(
                    "{} had more than two items!",
                    line
                ))),
            }
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
