use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::Context;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Trees(Vec<Vec<usize>>);

impl FromStr for Trees {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut field = vec![];
        for line in s.lines() {
            let mut field_line = vec![];
            for c in line.chars() {
                field_line.push(
                    c.to_digit(10)
                        .map(|i| i as usize)
                        .ok_or_else(|| anyhow::anyhow!("{} is not a digit!", c))?,
                );
            }
            field.push(field_line);
        }
        Ok(Trees(field))
    }
}

impl Trees {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn perimeter(&self) -> usize {
        4 * self.0.len() - 4
    }

    fn visible_from_north(&self) -> Vec<Vec<bool>> {
        let n = self.len();
        let mut highest = self.0[0].clone();
        let mut field = vec![vec![false; n]; n];
        for i in 1..(n - 1) {
            for j in 1..(n - 1) {
                let val = self.0[i][j];
                if val > highest[j] {
                    field[i][j] = true;
                    highest[j] = val;
                }
            }
        }
        field
    }

    fn visible_from_south(&self) -> Vec<Vec<bool>> {
        let n = self.len();
        let mut highest = self.0[n - 1].clone();
        let mut field = vec![vec![false; n]; n];
        for i in (1..(n - 1)).rev() {
            for j in 1..(n - 1) {
                let val = self.0[i][j];
                if val > highest[j] {
                    field[i][j] = true;
                    highest[j] = val;
                }
            }
        }
        field
    }

    fn visible_from_west(&self) -> Vec<Vec<bool>> {
        let n = self.len();

        let mut highest = vec![0usize; n];
        for i in 0..n {
            highest[i] = self.0[i][0];
        }

        let mut field = vec![vec![false; n]; n];
        for i in 1..(n - 1) {
            for j in 1..(n - 1) {
                let val = self.0[i][j];
                if val > highest[i] {
                    field[i][j] = true;
                    highest[i] = val;
                }
            }
        }
        field
    }

    fn visible_from_east(&self) -> Vec<Vec<bool>> {
        let n = self.len();

        let mut highest = vec![0usize; n];
        for i in 0..n {
            highest[i] = self.0[i][n - 1];
        }

        let mut field = vec![vec![false; n]; n];
        for i in 1..(n - 1) {
            for j in (1..(n - 1)).rev() {
                let val = self.0[i][j];
                if val > highest[i] {
                    field[i][j] = true;
                    highest[i] = val;
                }
            }
        }
        field
    }

    fn scenic_number(&self, i: usize, j: usize) -> usize {
        let n = self.len();

        let mut left = 0;
        let mut right = 0;
        for c in (0..j).rev() {
            left += 1;
            if self.0[i][c] >= self.0[i][j] {
                break;
            }
        }
        for c in (j + 1)..n {
            right += 1;
            if self.0[i][c] >= self.0[i][j] {
                break;
            }
        }

        let mut top = 0;
        let mut bottom = 0;
        for c in (0..i).rev() {
            top += 1;
            if self.0[c][j] >= self.0[i][j] {
                break;
            }
        }
        for c in (i + 1)..n {
            bottom += 1;
            if self.0[c][j] >= self.0[i][j] {
                break;
            }
        }

        left * right * top * bottom
    }
}

pub async fn part1(path: impl AsRef<Path>) -> anyhow::Result<usize> {
    let contents = tokio::fs::read_to_string(path).await?;

    let trees = Arc::new(Trees::from_str(&contents)?);
    let n = trees.len();

    let north = tokio::spawn({
        let trees = trees.clone();
        async move { trees.visible_from_north() }
    });
    let south = tokio::spawn({
        let trees = trees.clone();
        async move { trees.visible_from_south() }
    });
    let west = tokio::spawn({
        let trees = trees.clone();
        async move { trees.visible_from_west() }
    });
    let east = tokio::spawn({
        let trees = trees.clone();
        async move { trees.visible_from_east() }
    });

    let north = north.await.context("North task failed!")?;
    let south = south.await.context("South task failed!")?;
    let west = west.await.context("West task failed!")?;
    let east = east.await.context("East task failed!")?;

    let mut interior_visible_count = 0;
    for i in 1..(n - 1) {
        for j in 1..(n - 1) {
            if north[i][j] || south[i][j] || west[i][j] || east[i][j] {
                interior_visible_count += 1;
            }
        }
    }
    Ok(trees.perimeter() + interior_visible_count)
}

pub async fn part2(path: impl AsRef<Path>) -> anyhow::Result<usize> {
    let contents = tokio::fs::read_to_string(path).await?;

    let trees = Arc::new(Trees::from_str(&contents)?);
    let n = trees.len();

    futures::future::join_all((1..(n - 1)).cartesian_product(1..(n - 1)).map(|(i, j)| {
        let trees = trees.clone();
        tokio::spawn(async move { trees.scenic_number(i, j) })
    }))
    .await
    .into_iter()
    .collect::<Result<Vec<_>, _>>()
    .map(|v| {
        v.into_iter()
            .max()
            .ok_or_else(|| anyhow::anyhow!("No maximum could be found due to empty iterator!"))
    })
    .map_err(|err| err.into())
    .flatten()
}
