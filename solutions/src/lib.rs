use anyhow::Result;
mod day_1;
pub mod examples;
pub mod input;
pub mod solvers;


pub trait Solver {
    fn part_1(&self) -> Result<String>;
    fn part_2(&self) -> Result<String>;
}
