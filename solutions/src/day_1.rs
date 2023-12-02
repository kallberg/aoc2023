use anyhow::{Error, Result};
use thiserror::Error;

use crate::Solver;

#[derive(Default)]
pub struct Day {
    input: String,
}

static PATTERNS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

impl Day {
    fn find_both(index: usize, line: &str) -> Result<u32> {
        let first = line.chars().find_map(|c| c.to_digit(10));
        let last = line.chars().rev().find_map(|c| c.to_digit(10));

        let Some(first) = first else {
            let error = DayError::FirstMissing(index + 1);
            return Err(Error::from(error));
        };

        let Some(last) = last else {
            let error = DayError::LastMissing(index + 1);
            return Err(Error::from(error));
        };

        Ok(last + first * 10)
    }

    fn find_both_extended(index: usize, line: &str) -> Result<u32> {
        let first = Day::find_pattern(0..line.len(), line);
        let last = Day::find_pattern((0..line.len()).rev(), line);

        let Some(first) = first else {
            let error = DayError::FirstMissing(index + 1);
            return Err(Error::from(error));
        };

        let Some(last) = last else {
            let error = DayError::LastMissing(index + 1);
            return Err(Error::from(error));
        };

        Ok(last + first * 10)
    }

    fn find_pattern(mut iterator: impl Iterator<Item = usize>, line: &str) -> Option<u32> {
        iterator.find_map(|i| Day::digit_extended(&line[i..]))
    }
    fn digit_extended(window: &str) -> Option<u32> {
        for (index, pattern) in PATTERNS.iter().enumerate() {
            if window.starts_with(*pattern) {
                return Some((index + 1) as u32);
            }
        }

        let first_char = window.as_bytes()[0] as char;

        first_char.to_digit(10)
    }
}

#[derive(Error, Debug)]
pub enum DayError {
    #[error("first digit not found at input line {0}")]
    FirstMissing(usize),
    #[error("last digit not found at input line {0}")]
    LastMissing(usize),
}

impl Solver for Day {
    fn setup(&mut self, input: &str) {
        self.input = input.to_string();
    }

    fn parse(&mut self) -> Result<()> {
        Ok(())
    }

    fn part_1(&self) -> Result<String> {
        let input = &self.input;

        let mut sum: u32 = 0;

        for (index, line) in input.lines().enumerate() {
            sum += Day::find_both(index, line)?;
        }

        Ok(sum.to_string())
    }

    fn part_2(&self) -> Result<String> {
        let input = &self.input;

        let mut sum: u32 = 0;

        for (index, line) in input.lines().enumerate() {
            sum += Day::find_both_extended(index, line)?;
        }

        Ok(sum.to_string())
    }
}
