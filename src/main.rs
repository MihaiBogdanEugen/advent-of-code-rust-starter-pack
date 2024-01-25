use clap::Parser;
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Instant;

use crate::solution::DisplayableSolution;

pub mod puzzles;
pub mod solution;
pub mod year2015;
pub mod year2016;
pub mod year2017;
pub mod year2018;
pub mod year2019;
pub mod year2020;
pub mod year2021;
pub mod year2022;
pub mod year2023;

#[derive(Parser)]
#[command()]
struct Cli {
    #[arg(long, value_parser = clap::value_parser!(u16).range(2015..=2023))]
    year: u16,

    #[arg(long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,
}
#[allow(clippy::borrowed_box)]
fn main() {
    println!("==============");
    println!("Advent of Code");
    println!("==============");

    let cli: Cli = Cli::parse();
    let year: usize = cli.year.try_into().unwrap();
    let day: usize = cli.day.try_into().unwrap();

    let puzzles: Vec<Box<dyn DisplayableSolution>> = puzzles::for_year(year);
    let puzzle: &Box<dyn DisplayableSolution> = puzzles
        .get(day - 1)
        .unwrap_or_else(|| panic!("Cannot load puzzle of day {} of {}", day, year));

    let now: Instant = Instant::now();
    let solution1: String = puzzle.display_part1_solution();
    println!(
        "Year {year} | Day {day} | Part#1 | Duration {:?} | Result {solution1}",
        now.elapsed()
    );

    let now: Instant = Instant::now();
    let solution2: String = puzzle.display_part2_solution();
    println!(
        "Year {year} | Day {day} | Part#2 | Duration {:?} | Result {solution2}",
        now.elapsed()
    );
}

fn get_input(year: usize, day: usize) -> String {
    let path: PathBuf = PathBuf::from(format!("input/{year}/{day}.in"));
    if path.exists() {
        fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Cannot load input of day {} of {}", day, year))
    } else {
        panic!("Cannot find path of input of day {} of {}", day, year)
    }
}

fn to<T>(text: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    FromStr::from_str(text.trim()).unwrap()
}

fn is<T: FromStr>(text: &str) -> bool {
    text.trim().parse::<T>().is_ok()
}
