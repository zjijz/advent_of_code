use std::path::Path;
use std::str::FromStr;

use anyhow::Context;

#[derive(Debug, Clone)]
enum Instruction {
    Add(isize),
    Noop,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("addx") {
            s.get(4..)
                .ok_or_else(|| anyhow::anyhow!("Line was not at least 5 characters long!"))
                .map(str::trim)
                .and_then(|s| {
                    isize::from_str(s)
                        .with_context(|| format!("Could not parse {} into an isize.", s))
                })
                .map(Self::Add)
        } else if s.contains("noop") {
            Ok(Self::Noop)
        } else {
            Err(anyhow::anyhow!("{} was not `addx {{isize}}` or `noop`.", s))
        }
    }
}

#[derive(Debug, Clone)]
struct Signal<const N: usize> {
    cycles: [isize; N],
}

impl<const N: usize> Signal<N> {
    fn new(initial: isize, instructions: &[Instruction]) -> Self {
        let mut cycles = [initial; N];

        let mut value = initial;
        let mut cycle_i = 0;
        let mut instr_i = 0;
        let mut instr_stage = 0;
        while cycle_i < N {
            match &instructions[instr_i] {
                Instruction::Add(add) => match instr_stage {
                    0 => {
                        cycles[cycle_i] = value;

                        instr_stage = 1;
                    }
                    1 => {
                        cycles[cycle_i] = value;

                        value += add;
                        instr_stage = 0;
                        instr_i += 1;
                    }
                    _ => unreachable!(),
                },
                Instruction::Noop => {
                    cycles[cycle_i] = value;
                    instr_i += 1;
                }
            }
            cycle_i += 1;
        }

        Self { cycles }
    }

    fn signal_strength(&self, i: usize) -> anyhow::Result<isize> {
        if (i - 1) < N {
            Ok(i as isize * self.cycles[i - 1])
        } else {
            Err(anyhow::anyhow!("{} index out of bounds (max {}).", i, N))
        }
    }

    fn get(&self, i: usize) -> anyhow::Result<isize> {
        if (i - 1) < N {
            Ok(self.cycles[i - 1])
        } else {
            Err(anyhow::anyhow!("{} index out of bounds (max {}).", i, N))
        }
    }
}

pub async fn part1(path: impl AsRef<Path>) -> anyhow::Result<isize> {
    let contents = tokio::fs::read_to_string(path).await?;
    let instructions = contents
        .lines()
        .map(Instruction::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    let signal = Signal::<220>::new(1, &instructions);
    signal
        .signal_strength(20)
        .and_then(|i20| signal.signal_strength(60).map(move |i60| (i20, i60)))
        .and_then(|(i20, i60)| {
            signal
                .signal_strength(100)
                .map(move |i100| (i20, i60, i100))
        })
        .and_then(|(i20, i60, i100)| {
            signal
                .signal_strength(140)
                .map(move |i140| (i20, i60, i100, i140))
        })
        .and_then(|(i20, i60, i100, i140)| {
            signal
                .signal_strength(180)
                .map(move |i180| (i20, i60, i100, i140, i180))
        })
        .and_then(|(i20, i60, i100, i140, i180)| {
            signal
                .signal_strength(220)
                .map(move |i220| (i20, i60, i100, i140, i180, i220))
        })
        .map(|(i20, i60, i100, i140, i180, i220)| i20 + i60 + i100 + i140 + i180 + i220)
}

pub async fn part2(path: impl AsRef<Path>) -> anyhow::Result<String> {
    let contents = tokio::fs::read_to_string(path).await?;
    let instructions = contents
        .lines()
        .map(Instruction::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    let signal = Signal::<240>::new(1, &instructions);

    let mut output = String::with_capacity(240);
    for i in 0..240 {
        let value = signal.get(i + 1)?;
        output.push(
            if (value - 1) <= (i as isize % 40) && (i as isize % 40) <= (value + 1) {
                '#'
            } else {
                '.'
            },
        );
    }
    Ok(format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n",
        &output[0..40],
        &output[40..80],
        &output[80..120],
        &output[120..160],
        &output[160..200],
        &output[200..240],
    ))
}
