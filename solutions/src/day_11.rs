use crate::day_11::DataKind::EmptySpace;
use crate::Solver;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
struct Point2D {
    x: i32,
    y: i32,
}

impl Point2D {
    fn steps(&self, other: &Point2D) -> usize {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as usize
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum DataKind {
    EmptySpace,
    Galaxy(usize),
}

#[derive(Default)]
struct GiantImage {
    data: Vec<Vec<(Point2D, DataKind)>>,
}

impl GiantImage {
    fn expand(&mut self) {
        let mut galaxy_rows = vec![];
        let mut galaxy_columns = vec![];

        for (y, row) in self.data.iter_mut().enumerate() {
            while y >= galaxy_rows.len() {
                galaxy_rows.push(false);
            }

            for (x, (position, data)) in row.iter_mut().enumerate() {
                while x >= galaxy_columns.len() {
                    galaxy_columns.push(false);
                }

                if !EmptySpace.eq(data) {
                    galaxy_rows[y] = true;
                    galaxy_columns[x] = true;
                }
            }
        }

        for (x, galaxy_column) in galaxy_columns.into_iter().enumerate().rev() {
            if !galaxy_column {
                for y in 0..self.data.len() {
                    self.data[y].insert(x, ((Point2D { x: 0, y: 0 }, EmptySpace)))
                }
            }
        }

        for (y, galaxy_row) in galaxy_rows.into_iter().enumerate().rev() {
            if !galaxy_row {
                let mut row = vec![];
                for _x in 0..self.data[y].len() {
                    row.push(((Point2D { x: 0, y: 0 }, EmptySpace)))
                }
                self.data.insert(y, row);
            }
        }

        for (y, row) in self.data.iter_mut().enumerate() {
            for (x, (position, data)) in row.iter_mut().enumerate() {
                let y = y as i32;
                let x = x as i32;

                position.x = x;
                position.y = y;
            }
        }
    }

    fn galaxy_number_width(&self) -> usize {
        let max_galaxy_number = self
            .data
            .iter()
            .flatten()
            .filter_map(|(_, data)| match data {
                EmptySpace => None,
                DataKind::Galaxy(number) => Some(*number),
            })
            .max()
            .unwrap_or(0);

        max_galaxy_number.ilog10() as usize + 1
    }

    fn galaxy_positions(&self) -> Vec<Point2D> {
        self.data
            .iter()
            .flatten()
            .filter_map(|(pos, data)| match data {
                EmptySpace => None,
                DataKind::Galaxy(_) => Some(*pos),
            })
            .collect()
    }

    fn galaxy_pairs(&self) -> Vec<(Point2D, Point2D)> {
        let mut galaxy_positions = self.galaxy_positions();
        let mut pairs = vec![];

        while let Some(half) = galaxy_positions.pop() {
            for other in &galaxy_positions {
                pairs.push((half, *other))
            }
        }

        pairs
    }
}

impl Display for GiantImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let width = self.galaxy_number_width();

        for row in &self.data {
            for (_, data) in row {
                write!(
                    f,
                    "{:^width$}",
                    match data {
                        EmptySpace => ".".to_string(),
                        DataKind::Galaxy(number) => {
                            number.to_string()
                        }
                    },
                    width = width
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct Day {
    input: String,
    image: GiantImage,
}

impl Solver for Day {
    fn setup(&mut self, input: &str) {
        self.input = input.to_string();
    }

    fn parse(&mut self) -> anyhow::Result<()> {
        let mut galaxy_number = 0;

        for (y, line) in self.input.lines().enumerate() {
            let mut row = vec![];

            for (x, character) in line.chars().enumerate() {
                let x = x as i32;
                let y = y as i32;

                row.push(if character == '#' {
                    galaxy_number += 1;
                    ((Point2D { x, y }, DataKind::Galaxy(galaxy_number)))
                } else {
                    ((Point2D { x, y }), DataKind::EmptySpace)
                })
            }

            self.image.data.push(row);
        }

        self.image.expand();

        Ok(())
    }

    fn part_1(&self) -> anyhow::Result<String> {
        let mut total_steps = 0;

        for (left, right) in self.image.galaxy_pairs() {
            let steps = left.steps(&right);

            total_steps += steps;
        }

        Ok(total_steps.to_string())
    }

    fn part_2(&self) -> anyhow::Result<String> {
        Ok("Placeholder".into())
    }
}
