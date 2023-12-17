use crate::Solver;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
struct Point2D {
    x: u64,
    y: u64,
}

impl Point2D {
    fn steps(&self, other: &Point2D) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Default, Clone)]
struct GiantImage {
    width: u64,
    height: u64,
    galaxies: Vec<Point2D>,
}

impl GiantImage {
    fn expand(&mut self, amount: u64) {
        let mut offset = 0;
        for x in 0..self.width {
            if !self.column_has_galaxy(x + offset) {
                self.move_galaxies_by_x(x + offset, amount);
                offset += amount;
                self.width += amount;
            }
        }
        offset = 0;

        for y in 0..self.height {
            if !self.row_has_galaxy(y + offset) {
                self.move_galaxies_by_y(y + offset, amount);
                offset += amount;
                self.height += amount;
            }
        }
    }

    fn move_galaxies_by_x(&mut self, x: u64, amount: u64) {
        for galaxy in self.galaxies.iter_mut() {
            if galaxy.x >= x {
                galaxy.x += amount
            }
        }
    }

    fn move_galaxies_by_y(&mut self, y: u64, amount: u64) {
        for galaxy in self.galaxies.iter_mut() {
            if galaxy.y >= y {
                galaxy.y += amount
            }
        }
    }

    fn column_has_galaxy(&self, column: u64) -> bool {
        for galaxy in &self.galaxies {
            if galaxy.x == column {
                return true;
            }
        }

        false
    }

    fn row_has_galaxy(&self, row: u64) -> bool {
        for galaxy in &self.galaxies {
            if galaxy.y == row {
                return true;
            }
        }

        false
    }

    fn distance(&self) -> u64 {
        let mut total_steps = 0;

        for (index, galaxy) in self.galaxies.iter().enumerate() {
            for other_index in (index + 1)..self.galaxies.len() {
                let other = &self.galaxies[other_index];

                if !galaxy.eq(other) {
                    total_steps += galaxy.steps(other)
                }
            }
        }

        total_steps
    }
}

impl Display for GiantImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut galaxy_counter = 1;

        for y in 0..self.height {
            'outer: for x in 0..self.width {
                for galaxy in &self.galaxies {
                    if galaxy.x == x && galaxy.y == y {
                        write!(f, "{}", galaxy_counter)?;
                        galaxy_counter += 1;
                        continue 'outer;
                    }
                }

                write!(f, ".")?;
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
        for (y, line) in self.input.lines().enumerate() {
            for (x, character) in line.chars().enumerate() {
                if character == '#' {
                    if x as u64 >= self.image.width {
                        self.image.width = x as u64 + 1;
                    }
                    if y as u64 >= self.image.height {
                        self.image.height = y as u64 + 1;
                    }

                    self.image.galaxies.push(Point2D {
                        x: x as u64,
                        y: y as u64,
                    });
                }
            }
        }

        Ok(())
    }

    fn part_1(&self) -> anyhow::Result<String> {
        let mut image = self.image.clone();

        image.expand(1);

        Ok(image.distance().to_string())
    }

    fn part_2(&self) -> anyhow::Result<String> {
        let mut image = self.image.clone();

        image.expand(999_999);

        Ok(image.distance().to_string())
    }
}
