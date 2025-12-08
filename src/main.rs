mod aoc;
mod y2015;
mod y2025;

use clap::Parser;
use std::fs;

#[derive(Debug, clap::Parser)]
struct Cli {
    year: u8,
    day: u8,
    part: Part,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
enum Part {
    #[clap(name = "1")]
    Part1,
    #[clap(name = "2")]
    Part2,
}

fn main() {
    let cli = Cli::parse();
    println!("{cli:?}");
    let year: &str = &format!("20{:02}", cli.year);
    let day: &str = &format!("{:02}", cli.day);
    let part2 = cli.part == Part::Part2;

    let input = fs::read_to_string(format!("inputs/{year}/day{day}.txt"))
        .expect("Forgot to add the input or wrong filename");

    dispatch!(year, day, input, part2, {
        "2025" => {
            "01" => y2025::day01,
            "02" => y2025::day02,
            "03" => y2025::day03,
            "04" => y2025::day04,
            "05" => y2025::day05,
            "06" => y2025::day06,
            "07" => y2025::day07
        },
        "2015" => {
            "01" => y2015::day01
        }
    });
}
