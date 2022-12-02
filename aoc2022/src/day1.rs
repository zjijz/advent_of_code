use futures::TryFutureExt as _;
use std::path::Path;

fn calculate_elves<'a>(
    contents: &'a str,
) -> impl Iterator<
    Item = futures::future::UnwrapOrElse<
        tokio::task::JoinHandle<usize>,
        impl FnOnce(tokio::task::JoinError) -> usize,
    >,
> + 'a {
    contents.split("\n\n").map(|group| {
        let group = group.to_string();
        tokio::spawn(async move {
            group
                .split("\n")
                .map(|item| str::parse::<usize>(item).unwrap())
                .sum::<usize>()
        })
        .unwrap_or_else(|_| 0usize)
    })
}

pub async fn part1(path: impl AsRef<Path>) -> anyhow::Result<usize> {
    let contents = tokio::fs::read_to_string(path).await?;
    let elves = calculate_elves(&contents);
    futures::future::join_all(elves)
        .await
        .into_iter()
        .max()
        .ok_or(anyhow::anyhow!("File had no contents!"))
}

pub async fn part2(path: impl AsRef<Path>) -> anyhow::Result<usize> {
    let contents = tokio::fs::read_to_string(path).await?;
    let elves = calculate_elves(&contents);

    let mut bundles = futures::future::join_all(elves).await;
    if bundles.len() < 3 {
        anyhow::bail!("Too few elves: {}.", bundles.len());
    }
    bundles.sort_unstable();
    Ok(bundles[(bundles.len() - 3)..bundles.len()]
        .into_iter()
        .sum::<usize>())
}
