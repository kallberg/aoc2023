use crate::Solver;
use anyhow::{Error, Result};
use thiserror::Error;

type GameSet = (u32, u32, u32);

#[derive(Default)]
pub struct Day {
    input: String,
    rgb_game_max: Vec<(u32, u32, u32)>,
}

#[derive(Error, Debug)]
pub enum DayError {
    #[error("split game set by whitespace delimiter")]
    GameSetDelimiter,
    #[error("parse game set integer string {0}")]
    GameSetInteger(String),
    #[error("map game set color {0}")]
    GameSetColor(String),
}

impl Day {
    fn parse_game_set(value: &str) -> Result<GameSet> {
        let (amount_str, color) = value.split_once(" ").ok_or(DayError::GameSetDelimiter)?;

        let amount: u32 = amount_str
            .parse()
            .map_err(|_err| DayError::GameSetInteger(amount_str.to_string()))?;

        match color {
            "blue" => Ok((0, 0, amount)),
            "red" => Ok((amount, 0, 0)),
            "green" => Ok((0, amount, 0)),
            _ => Err(Error::from(DayError::GameSetColor(color.to_string()))),
        }
    }

    fn line_game_set(set: &str) -> Result<Vec<GameSet>> {
        let mut reveals = vec![];
        let cubes = set.split(",");

        for cube in cubes {
            reveals.push(Day::parse_game_set(cube.trim())?);
        }

        Ok(reveals)
    }
    fn line_reveals(line: &str) -> Result<Vec<GameSet>> {
        let line = line.split_once(":").unwrap().1;
        let sets = line.split(";");

        let mut game_sets = vec![];

        for set in sets {
            game_sets.append(&mut Day::line_game_set(set)?)
        }

        Ok(game_sets)
    }

    fn games(input: &str) -> Result<Vec<Vec<GameSet>>> {
        let mut lines_reveals = vec![];

        for line in input.lines() {
            let line_reveals = Day::line_reveals(line)?;

            lines_reveals.push(line_reveals);
        }

        Ok(lines_reveals)
    }

    fn game_max_rgb(line: Vec<GameSet>) -> (u32, u32, u32) {
        line.into_iter()
            .reduce(|(ar, ag, ab), (r, g, b)| (ar.max(r), ag.max(g), ab.max(b)))
            .unwrap_or((0, 0, 0))
    }
}

impl Solver for Day {
    fn setup(&mut self, input: &str) {
        self.input = input.to_string();
    }

    fn parse(&mut self) -> Result<()> {
        let games = Day::games(&self.input)?;

        for game in games.into_iter() {
            self.rgb_game_max.push(Day::game_max_rgb(game))
        }

        Ok(())
    }

    fn part_1(&self) -> anyhow::Result<String> {
        let mut id_sum = 0;

        for (index, (r, g, b)) in self.rgb_game_max.iter().enumerate() {
            let id = index + 1;
            if *r > 12 || *g > 13 || *b > 14 {
                continue;
            }
            id_sum += id;
        }

        Ok(id_sum.to_string())
    }

    fn part_2(&self) -> anyhow::Result<String> {
        let mut power_sum = 0;

        for (r, g, b) in self.rgb_game_max.iter() {
            power_sum += r * g * b;
        }

        Ok(power_sum.to_string())
    }
}
