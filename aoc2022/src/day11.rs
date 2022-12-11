use std::collections::VecDeque;
use std::path::Path;
use std::str::FromStr;

use anyhow::Context;

#[derive(Debug)]
enum Expr {
    Add(usize),
    Mul(usize),
    Square,
}

impl Expr {
    fn eval(&self, val: usize) -> usize {
        match self {
            Self::Add(other) => val.wrapping_add(*other),
            Self::Mul(other) => val.wrapping_mul(*other),
            Self::Square => val.wrapping_pow(2),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Expr,
    test_divisible_by: usize,
    on_success_monkey_ind: usize,
    on_failure_monkey_ind: usize,
    inspection_count: usize,
}

#[derive(Debug)]
struct Monkeys {
    monkeys: Vec<Monkey>,
}

fn parse_monkey_line_0(s: &str) -> anyhow::Result<usize> {
    let c = s
        .chars()
        .nth(7)
        .ok_or_else(|| anyhow::anyhow!("Line did not have at least 8 chars."))?;
    c.to_digit(10)
        .map(|i| i as usize)
        .ok_or_else(|| anyhow::anyhow!("Char could not be parsed into a digit."))
}

fn parse_monkey_line_1(s: &str) -> anyhow::Result<VecDeque<usize>> {
    Ok(s.get(17..)
        .ok_or_else(|| anyhow::anyhow!("Line did not have at least 18 characters."))?
        .split(",")
        .map(str::trim)
        .map(usize::from_str)
        .collect::<Result<VecDeque<_>, _>>()
        .context("Could not convert str to usize.")?)
}

fn parse_monkey_line_2(s: &str) -> anyhow::Result<Expr> {
    let op = s
        .chars()
        .nth(23)
        .ok_or_else(|| anyhow::anyhow!("Line did have at least 24 characters!"))?;
    let argument = s
        .get(24..)
        .ok_or_else(|| anyhow::anyhow!("Line did not have at least 25 characters!"))
        .map(str::trim)?;
    match (op, argument) {
        ('+', arg) => arg.parse::<usize>().map(Expr::Add).with_context(|| {
            format!(
                "Argument to addition operation could not be parsed to a usize: {}.",
                arg,
            )
        }),
        ('*', "old") => Ok(Expr::Square),
        ('*', arg) => arg.parse::<usize>().map(Expr::Mul).with_context(|| {
            format!(
                "Argument to multiplication operation could not be parsed to a usize: {}.",
                arg,
            )
        }),
        _ => anyhow::bail!("Unrecognized operator {}!", op),
    }
}

fn parse_monkey_line_3(s: &str) -> anyhow::Result<usize> {
    s.get(21..)
        .ok_or_else(|| anyhow::anyhow!("Line did not have 22 characters."))
        .and_then(|s| {
            usize::from_str(s).with_context(|| format!("Could not parse {} into usize.", s))
        })
}

fn parse_monkey_line_4(s: &str) -> anyhow::Result<usize> {
    s.chars()
        .nth(29)
        .ok_or_else(|| anyhow::anyhow!("Line did not have 30 characters."))
        .and_then(|c| {
            c.to_digit(10)
                .with_context(|| format!("Could not parse {} into a base-10 digit.", c))
        })
        .map(|i| i as usize)
}

fn parse_monkey_line_5(s: &str) -> anyhow::Result<usize> {
    s.chars()
        .nth(30)
        .ok_or_else(|| anyhow::anyhow!("Line did not have 31 characters."))
        .and_then(|c| {
            c.to_digit(10)
                .with_context(|| format!("Could not parse {} into a base-10 digit.", c))
        })
        .map(|i| i as usize)
}

impl FromStr for Monkeys {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Monkeys {
            monkeys: s
                .split("\n\n")
                .enumerate()
                .map(|(i, item)| -> anyhow::Result<Monkey> {
                    let lines = item.trim().lines().collect::<Vec<_>>();

                    let index =
                        parse_monkey_line_0(item).context("Could not parse monkey line 0!")?;
                    anyhow::ensure!(
                        index == i,
                        "Monkey parsing index is not what is expected! {} vs {}",
                        index,
                        i,
                    );

                    Ok(Monkey {
                        items: parse_monkey_line_1(lines[1])
                            .context("Could not parse monkey line 1!")?,
                        operation: parse_monkey_line_2(lines[2])
                            .context("Could not parse monkey line 2!")?,
                        test_divisible_by: parse_monkey_line_3(lines[3])
                            .context("Could not parse monkey line 3!")?,
                        on_success_monkey_ind: parse_monkey_line_4(lines[4])
                            .context("Could not parse monkey line 4!")?,
                        on_failure_monkey_ind: parse_monkey_line_5(lines[5])
                            .context("Could not parse monkey line 5!")?,
                        inspection_count: 0,
                    })
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl Monkeys {
    fn lcm(&self) -> usize {
        // N.B.: This works as lcm because all of the divisors are prime.
        self.monkeys
            .iter()
            .map(
                |Monkey {
                     test_divisible_by, ..
                 }| test_divisible_by,
            )
            .product()
    }

    fn simulate<const PART: usize>(&mut self) {
        let lcm = self.lcm();

        let mut temp_items = vec![VecDeque::<usize>::new(); self.monkeys.len()];

        // Simulate the current round by applying the monkey operations and tests in order.
        for i in 0..(self.monkeys.len()) {
            let monkey = &mut self.monkeys[i];
            monkey.items.extend(temp_items[i].drain(..));
            for item in monkey.items.drain(..) {
                // Increment expection count.
                monkey.inspection_count += 1;

                // Apply operation.
                let mut new_val = monkey.operation.eval(item);
                if PART == 1 {
                    // Divide by 3 for monkey getting bored. This is integer division.
                    new_val /= 3;
                } else {
                    // Divide by LCM of all Monkey divisors. Oh, because they
                    // are all prime and this can't change the answer since each
                    // test is a prime modulo.
                    new_val %= lcm;
                }

                // Run the test to see the next monkey.
                if new_val % monkey.test_divisible_by == 0 {
                    temp_items[monkey.on_success_monkey_ind].push_back(new_val);
                } else {
                    temp_items[monkey.on_failure_monkey_ind].push_back(new_val);
                }
            }
        }

        // Add the remaining temp_items to the monkey queues for the next round.
        for i in 0..self.monkeys.len() {
            self.monkeys[i].items.extend(temp_items[i].drain(..));
        }
    }

    fn print_monkey_inspection_counts(&self) {
        println!(
            "{:#?}",
            self.monkeys
                .iter()
                .map(
                    |Monkey {
                         inspection_count, ..
                     }| *inspection_count,
                )
                .collect::<Vec<_>>()
        )
    }

    fn monkey_business_level(&self) -> usize {
        let (first, second) = self
            .monkeys
            .iter()
            .map(
                |Monkey {
                     inspection_count, ..
                 }| *inspection_count,
            )
            .fold((0, 0), |(first, second), count| {
                if count > first {
                    (count, first)
                } else if count > second {
                    (first, count)
                } else {
                    (first, second)
                }
            });
        first * second
    }
}

pub async fn part<const PART: usize, const ROUNDS: usize>(
    path: impl AsRef<Path>,
) -> anyhow::Result<usize> {
    let contents = tokio::fs::read_to_string(path).await?;
    let mut monkeys = Monkeys::from_str(&contents).context("Could not parse Monkeys from file.")?;
    for _ in 0..ROUNDS {
        monkeys.simulate::<PART>();
    }
    monkeys.print_monkey_inspection_counts();
    Ok(monkeys.monkey_business_level())
}
