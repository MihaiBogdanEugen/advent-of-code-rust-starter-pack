use crate::solution::DisplayableSolution;
use crate::year2015;
use crate::year2016;
use crate::year2017;
use crate::year2018;
use crate::year2019;
use crate::year2020;
use crate::year2021;
use crate::year2022;
use crate::year2023;

#[macro_export]
macro_rules! puzzle {
    ($year:expr, $day:expr) => {
        use $crate::get_input;

        pub struct Puzzle {
            input: String,
        }

        impl Default for Puzzle {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Puzzle {
            pub fn new() -> Self {
                Puzzle {
                    input: get_input($year, $day),
                }
            }

            #[allow(dead_code)]
            pub fn with_input(input: &str) -> Self {
                Puzzle {
                    input: input.to_string(),
                }
            }
        }

        impl DisplayableSolution for Puzzle {
            fn display_part1_solution(&self) -> String {
                self.solve_part1().to_string()
            }

            fn display_part2_solution(&self) -> String {
                self.solve_part2().to_string()
            }
        }
    };
}

pub fn for_year(year: usize) -> Vec<Box<dyn DisplayableSolution>> {
    match year {
        2015 => year2015::puzzles(),
        2016 => year2016::puzzles(),
        2017 => year2017::puzzles(),
        2018 => year2018::puzzles(),
        2019 => year2019::puzzles(),
        2020 => year2020::puzzles(),
        2021 => year2021::puzzles(),
        2022 => year2022::puzzles(),
        2023 => year2023::puzzles(),
        _ => unreachable!(),
    }
}
