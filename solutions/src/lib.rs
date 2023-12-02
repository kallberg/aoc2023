use anyhow::Result;
mod day_1;
mod day_2;
pub mod examples;
pub mod input;
pub mod solvers;

pub trait Solver {
    fn setup(&mut self, input: &str);
    fn parse(&mut self) -> Result<()>;
    fn part_1(&self) -> Result<String>;
    fn part_2(&self) -> Result<String>;
}
