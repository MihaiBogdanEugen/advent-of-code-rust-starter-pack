pub mod day01;
// pub mod day02;
// pub mod day03;
// pub mod day04;
// pub mod day05;
// pub mod day06;
// pub mod day07;
// pub mod day08;
// pub mod day09;
// pub mod day10;
// pub mod day11;
// pub mod day12;
// pub mod day13;
// pub mod day14;
// pub mod day15;
// pub mod day16;
// pub mod day17;
// pub mod day18;
// pub mod day19;
// pub mod day20;
// pub mod day21;
// pub mod day22;
// pub mod day23;
// pub mod day24;
// pub mod day25;

use crate::solution::DisplayableSolution;
use crate::solution::NoSolution;

pub fn puzzles() -> Vec<Box<dyn DisplayableSolution>> {
    let mut puzzles: Vec<Box<dyn DisplayableSolution>> = Vec::new();
    for _ in 0..25 {
        puzzles.push(Box::new(NoSolution {}));
    }

    puzzles[0] = Box::new(day01::Puzzle::new());
    // puzzles[1] = Box::new(day02::Puzzle::new());
    // puzzles[2] = Box::new(day03::Puzzle::new());
    // puzzles[3] = Box::new(day04::Puzzle::new());
    // puzzles[4] = Box::new(day05::Puzzle::new());
    // puzzles[5] = Box::new(day06::Puzzle::new());
    // puzzles[6] = Box::new(day07::Puzzle::new());
    // puzzles[7] = Box::new(day08::Puzzle::new());
    // puzzles[8] = Box::new(day09::Puzzle::new());
    // puzzles[9] = Box::new(day10::Puzzle::new());
    // puzzles[10] = Box::new(day11::Puzzle::new());
    // puzzles[11] = Box::new(day12::Puzzle::new());
    // puzzles[12] = Box::new(day13::Puzzle::new());
    // puzzles[13] = Box::new(day14::Puzzle::new());
    // puzzles[14] = Box::new(day15::Puzzle::new());
    // puzzles[15] = Box::new(day16::Puzzle::new());
    // puzzles[16] = Box::new(day17::Puzzle::new());
    // puzzles[17] = Box::new(day18::Puzzle::new());
    // puzzles[18] = Box::new(day19::Puzzle::new());
    // puzzles[19] = Box::new(day20::Puzzle::new());
    // puzzles[20] = Box::new(day21::Puzzle::new());
    // puzzles[21] = Box::new(day22::Puzzle::new());
    // puzzles[22] = Box::new(day23::Puzzle::new());
    // puzzles[23] = Box::new(day24::Puzzle::new());
    // puzzles[24] = Box::new(day25::Puzzle::new());

    puzzles
}
