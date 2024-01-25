use std::fmt::Display;

pub trait Solution<T>
where
    T: Display + ?Sized,
{
    fn solve_part1(&self) -> T;
    fn solve_part2(&self) -> T;
}

pub trait DisplayableSolution {
    fn display_part1_solution(&self) -> String;
    fn display_part2_solution(&self) -> String;
}

pub struct NoSolution {}

impl DisplayableSolution for NoSolution {
    fn display_part1_solution(&self) -> String {
        unimplemented!("No solution yet for part #1!");
    }

    fn display_part2_solution(&self) -> String {
        unimplemented!("No solution yet for part #2!");
    }
}
