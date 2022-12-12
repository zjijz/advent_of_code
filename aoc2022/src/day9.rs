use std::collections::HashSet;
use std::path::Path;
use std::str::FromStr;

use anyhow::Context as _;

#[derive(Debug)]
enum Move {
    Left(usize),
    Up(usize),
    Right(usize),
    Down(usize),
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ty = s
            .chars()
            .nth(0)
            .ok_or_else(|| anyhow::anyhow!("Line did not have any characters!"))?;
        let count = s
            .get(2..)
            .ok_or_else(|| anyhow::anyhow!(""))
            .map(str::trim)
            .and_then(|s| {
                usize::from_str(s).with_context(|| format!("Could not parse {} into a usize.", s))
            })?;
        match ty {
            'L' => Ok(Move::Left(count)),
            'U' => Ok(Move::Up(count)),
            'R' => Ok(Move::Right(count)),
            'D' => Ok(Move::Down(count)),
            other => anyhow::bail!("Unknown Move type: {}.", other),
        }
    }
}

type Point = (usize, usize);

fn move_knot_to_knot(first: Point, second: Point) -> Point {
    let mut res = second;
    if first.0.abs_diff(second.0) > 1 || first.1.abs_diff(second.1) > 1 {
        if first.0 == second.0 {
            // head.1 != tail.1
            if first.1 + 1 > second.1 {
                res.1 += 1;
            } else if first.1 < second.1 + 1 {
                res.1 -= 1;
            }
        } else if first.1 == second.1 {
            // head.0 != tail.0
            if first.0 + 1 > second.0 {
                res.0 += 1;
            } else if first.0 < second.0 + 1 {
                res.0 -= 1;
            }
        } else {
            // head.0 != tail.0 && head.1 != tail.1
            if first.0 > second.0 {
                res.0 += 1;
            } else {
                // head.0 < tail.0
                res.0 -= 1;
            }

            if first.1 > second.1 {
                res.1 += 1;
            } else {
                // head.1 < tail.1
                res.1 -= 1;
            }
        }
    }
    res
}

macro move_impl_1($head:ident, $tail:ident, $tail_visited:ident, $new_head:expr, $amount:expr $(,)?) {
    for _ in 0..$amount {
        $head = $new_head;
        $tail = move_knot_to_knot($head, $tail);
        $tail_visited.insert($tail);
    }
}

pub async fn part1(path: impl AsRef<Path>) -> anyhow::Result<usize> {
    let contents = tokio::fs::read_to_string(path).await?;
    let moves = contents
        .lines()
        .map(Move::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    let mut head = (1000, 1000);
    let mut tail = (1000, 1000);
    let mut tail_visited = vec![tail].into_iter().collect::<HashSet<_>>();
    for m in moves {
        match m {
            Move::Left(amount) => {
                move_impl_1!(head, tail, tail_visited, (head.0 - 1, head.1), amount)
            }
            Move::Up(amount) => {
                move_impl_1!(head, tail, tail_visited, (head.0, head.1 + 1), amount)
            }
            Move::Right(amount) => {
                move_impl_1!(head, tail, tail_visited, (head.0 + 1, head.1), amount)
            }
            Move::Down(amount) => {
                move_impl_1!(head, tail, tail_visited, (head.0, head.1 - 1), amount)
            }
        }
    }
    Ok(tail_visited.len())
}

macro move_impl_2(
    $head:ident, 
    $tail1:ident, 
    $tail2:ident, 
    $tail3:ident, 
    $tail4:ident, 
    $tail5:ident, 
    $tail6:ident, 
    $tail7:ident, 
    $tail8:ident,
    $tail9:ident,
    $tail_visited:ident, 
    $new_head:expr,
    $amount:expr $(,)?
) {
    for _ in 0..$amount {
        $head = $new_head;
        $tail1 = move_knot_to_knot($head, $tail1);
        $tail2 = move_knot_to_knot($tail1, $tail2);
        $tail3 = move_knot_to_knot($tail2, $tail3);
        $tail4 = move_knot_to_knot($tail3, $tail4);
        $tail5 = move_knot_to_knot($tail4, $tail5);
        $tail6 = move_knot_to_knot($tail5, $tail6);
        $tail7 = move_knot_to_knot($tail6, $tail7);
        $tail8 = move_knot_to_knot($tail7, $tail8);
        $tail9 = move_knot_to_knot($tail8, $tail9);
        $tail_visited.insert($tail9);
    }
}

pub async fn part2(path: impl AsRef<Path>) -> anyhow::Result<usize> {
    let contents = tokio::fs::read_to_string(path).await?;
    let moves = contents
        .lines()
        .map(Move::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    let mut head = (1000, 1000);
    let mut tail1 = head;
    let mut tail2 = head;
    let mut tail3 = head;
    let mut tail4 = head;
    let mut tail5 = head;
    let mut tail6 = head;
    let mut tail7 = head;
    let mut tail8 = head;
    let mut tail9 = head;
    let mut tail_visited = vec![tail9].into_iter().collect::<HashSet<_>>();
    for m in moves {
        match m {
            Move::Left(amount) => {
                move_impl_2!(
                    head, 
                    tail1, tail2, tail3, tail4, tail5, tail6, tail7, tail8, tail9,
                    tail_visited, 
                    (head.0 - 1, head.1),
                    amount,
                )
            }
            Move::Up(amount) => {
                move_impl_2!(
                    head, 
                    tail1, tail2, tail3, tail4, tail5, tail6, tail7, tail8, tail9,
                    tail_visited, 
                    (head.0, head.1 + 1),
                    amount,
                )
            }
            Move::Right(amount) => {
                move_impl_2!(
                    head, 
                    tail1, tail2, tail3, tail4, tail5, tail6, tail7, tail8, tail9,
                    tail_visited, 
                    (head.0 + 1, head.1), 
                    amount,
                )
            }
            Move::Down(amount) => {
                move_impl_2!(
                    head, 
                    tail1, tail2, tail3, tail4, tail5, tail6, tail7, tail8, tail9,
                    tail_visited, 
                    (head.0, head.1 - 1), 
                    amount,
                )
            }
        }
    }
    Ok(tail_visited.len())
}
