use crate::{day_1, day_2, day_3, Solver};

pub struct Solvers;

impl Solvers {
    pub fn get(day: usize) -> Option<Box<dyn Solver>> {
        match day {
            1 => {
                let day = day_1::Day::default();
                let boxed: Box<dyn Solver> = Box::new(day);
                Some(boxed)
            }
            2 => {
                let day: day_2::Day = day_2::Day::default();
                let boxed: Box<dyn Solver> = Box::new(day);
                Some(boxed)
            }
            3 => Some(Box::new(day_3::Day::default())),
            _ => None,
        }
    }
}
