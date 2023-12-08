use anyhow::Result;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
pub mod example;
pub mod input;
pub mod solvers;

pub trait Solver {
    fn setup(&mut self, input: &str);
    fn parse(&mut self) -> Result<()>;
    fn part_1(&self) -> Result<String>;
    fn part_2(&self) -> Result<String>;
}
