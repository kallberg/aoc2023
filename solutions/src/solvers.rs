use crate::{day_1, day_2, Solver};

pub struct Solvers;

impl Solvers {
    pub fn get(day: usize, input: &str) -> Option<Box<dyn Solver>> {
        match day {
            1 => {
                let day = day_1::Day::from(input);
                let boxed: Box<dyn Solver> = Box::new(day);
                Some(boxed)
            },
            2 => {
                let day = day_2::Day::from(input);
                let boxed: Box<dyn Solver> = Box::new(day);
                Some(boxed)
            }
            _ => None
        }
    }
}
