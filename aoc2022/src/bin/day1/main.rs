use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    file_path: PathBuf,
}

#[tokio::main]
async fn main() {
    println!(
        "Part 1: {:#?}",
        aoc2022::day1::part1(Args::from_args().file_path).await
    );

    println!(
        "Part 2: {:#?}",
        aoc2022::day1::part2(Args::from_args().file_path).await
    );
}
