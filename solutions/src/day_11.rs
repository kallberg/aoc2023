use crate::Solver;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
struct Point2D {
    x: usize,
    y: usize,
}

impl Point2D {
    fn steps(&self, other: &Point2D) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Default, Clone)]
struct GiantImage {
    width: usize,
    height: usize,
    galaxies: Vec<Point2D>,
}

impl GiantImage {
    fn expand(&mut self, amount: usize) {
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

    fn move_galaxies_by_x(&mut self, x: usize, amount: usize) {
        for galaxy in self.galaxies.iter_mut() {
            if galaxy.x >= x {
                galaxy.x += amount
            }
        }
    }

    fn move_galaxies_by_y(&mut self, y: usize, amount: usize) {
        for galaxy in self.galaxies.iter_mut() {
            if galaxy.y >= y {
                galaxy.y += amount
            }
        }
    }

    fn column_has_galaxy(&self, column: usize) -> bool {
        for galaxy in &self.galaxies {
            if galaxy.x == column {
                return true;
            }
        }

        false
    }

    fn row_has_galaxy(&self, row: usize) -> bool {
        for galaxy in &self.galaxies {
            if galaxy.y == row {
                return true;
            }
        }

        false
    }

    fn distance(&self) -> usize {
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
                    if x >= self.image.width {
                        self.image.width = x + 1;
                    }
                    if y >= self.image.height {
                        self.image.height = y + 1;
                    }

                    self.image.galaxies.push(Point2D { x, y });
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
