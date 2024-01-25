use crate::puzzle;
use crate::solution::DisplayableSolution;
use crate::solution::Solution;

puzzle!(2015, 1);

impl Solution<isize> for Puzzle {
    fn solve_part1(&self) -> isize {
        0
    }

    fn solve_part2(&self) -> isize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_simple_input() {
        assert_eq!(0, Puzzle::with_input(r#""#).solve_part1());
    }

    #[test]
    fn test_part2_simple_input() {
        assert_eq!(0, Puzzle::with_input(r#""#).solve_part2());
    }

    #[test]
    fn test_part1_official_input() {
        assert_eq!(0, Puzzle::new().solve_part1());
    }

    #[test]
    fn test_part2_official_input() {
        assert_eq!(0, Puzzle::new().solve_part2());
    }
}
