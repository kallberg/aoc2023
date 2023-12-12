use crate::{day_1, day_2, day_3, day_4, day_5, day_6, day_7, day_8, day_9, day_x, Solver};

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
            7 => Some(Box::new(day_7::Day::default())),
            8 => Some(Box::new(day_8::Day::default())),
            9 => Some(Box::new(day_9::Day::default())),
            10 => Some(Box::new(day_x::Day::default())),
            11 => Some(Box::new(day_x::Day::default())),
            12 => Some(Box::new(day_x::Day::default())),
            13 => Some(Box::new(day_x::Day::default())),
            14 => Some(Box::new(day_x::Day::default())),
            15 => Some(Box::new(day_x::Day::default())),
            16 => Some(Box::new(day_x::Day::default())),
            17 => Some(Box::new(day_x::Day::default())),
            18 => Some(Box::new(day_x::Day::default())),
            19 => Some(Box::new(day_x::Day::default())),
            20 => Some(Box::new(day_x::Day::default())),
            21 => Some(Box::new(day_x::Day::default())),
            22 => Some(Box::new(day_x::Day::default())),
            23 => Some(Box::new(day_x::Day::default())),
            24 => Some(Box::new(day_x::Day::default())),
            25 => Some(Box::new(day_x::Day::default())),
            _ => None,
        }
    }
}
