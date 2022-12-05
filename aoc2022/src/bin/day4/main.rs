use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    file_path: PathBuf,
}

fn main() {
    println!(
        "Part 1: {:#?}",
        aoc2022::day4::part1(Args::from_args().file_path),
    );

    println!(
        "Part 2: {:#?}",
        aoc2022::day4::part2(Args::from_args().file_path),
    );
}
