use anyhow::{Error, Result};
use thiserror::Error;

use crate::Solver;
pub struct Day {
    input: String,
}

impl From<&str> for Day {
    fn from(value: &str) -> Self {
        Self {
            input: value.into(),
        }
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
    fn part_1(&self) -> Result<String> {
        let input = &self.input;

        let mut sum: u16 = 0;

        for (index, line) in input.lines().enumerate() {
            let digits: Vec<char> = line.chars().filter(|char| char.is_digit(10)).collect();

            let Some(first) = digits.first() else {
                let error = DayError::FirstMissing(index + 1);
                return Err(Error::from(error));
            };

            let Some(last) = digits.last() else {
                let error = DayError::LastMissing(index + 1);
                return Err(Error::from(error));
            };

            let mut string = String::new();

            string.push(*first);
            string.push(*last);

            let value = string.parse::<u16>()?;

            sum += value
        }

        Ok(sum.to_string())
    }

    fn part_2(&self) -> Result<String> {
        let input = &self.input;

        let patterns: Vec<Vec<char>> = vec![
            vec!['o', 'n', 'e'],
            vec!['t', 'w', 'o'],
            vec!['t', 'h', 'r', 'e', 'e'],
            vec!['f', 'o', 'u', 'r'],
            vec!['f', 'i', 'v', 'e'],
            vec!['s', 'i', 'x'],
            vec!['s', 'e', 'v', 'e', 'n'],
            vec!['e', 'i', 'g', 'h', 't'],
            vec!['n', 'i', 'n', 'e'],
        ];

        fn digit(window: &[char], patterns: &Vec<Vec<char>>) -> Option<u32> {
            if window[0].is_digit(10) {
                return window[0].to_digit(10);
            }

            for (index, pattern) in patterns.iter().enumerate() {
                if window.starts_with(&pattern) {
                    return Some((index + 1) as u32);
                }
            }

            None
        }

        let mut sum: u32 = 0;

        for (index, line) in input.lines().enumerate() {
            let chars: Vec<char> = line.chars().into_iter().collect();
            let mut cursor: usize = 0;

            let mut first = None;
            let mut current = None;

            let max_index = chars.len() - 1;

            while cursor < chars.len() {
                let end = max_index.min(cursor + 5);
                let window = &chars[cursor..=end];

                if window.is_empty() {
                    break;
                }

                if let Some(digit) = digit(window, &patterns) {
                    current = Some(digit);

                    if first.is_none() {
                        first = Some(digit);
                    }
                }

                cursor += 1;
            }

            let Some(first) = first else {
                let error = DayError::FirstMissing(index + 1);
                return Err(Error::from(error));
            };

            let Some(last) = current else {
                let error = DayError::LastMissing(index + 1);
                return Err(Error::from(error));
            };

            sum += first * 10;
            sum += last;
        }

        Ok(sum.to_string())
    }
}
