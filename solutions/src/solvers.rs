use crate::{day_1, day_2, day_3, day_4, day_5, day_6, Solver};

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
            4 => Some(Box::new(day_4::Day::default())),
            5 => Some(Box::new(day_5::Day::default())),
            6 => Some(Box::new(day_6::Day::default())),
            _ => None,
        }
    }
}
