use crate::Solver;

#[derive(Debug)]
struct PartNumber {
    value: u32,
    width: usize,
    x: u32,
    y: u32,
}

impl PartNumber {
    fn adjacent(&self, x: u32, y: u32) -> bool {
        let dy = y.abs_diff(self.y);

        if dy > 1 {
            return false;
        }

        if x + 1 < self.x {
            return false;
        }

        if x > self.x + self.width as u32 {
            return false;
        }

        true
    }
}

#[derive(Default)]
pub struct Day {
    input: String,
    part_numbers: Vec<PartNumber>,
    symbols: Vec<(u32, u32)>,
    gears: Vec<(u32, u32)>,
}

impl Solver for Day {
    fn setup(&mut self, input: &str) {
        self.input = input.to_string();
    }

    fn parse(&mut self) -> anyhow::Result<()> {
        for (y, line) in self.input.lines().enumerate() {
            let mut reading_digits = false;
            let mut part_value: u32 = 0;

            for (x, char) in line.chars().enumerate() {
                if reading_digits && !char.is_digit(10) {
                    let width = (part_value.ilog10() + 1) as usize;
                    let part_x = x as u32 - width as u32;

                    let part = PartNumber {
                        value: part_value,
                        width,
                        x: part_x,
                        y: y as u32,
                    };

                    self.part_numbers.push(part);
                    reading_digits = false;
                    part_value = 0;
                }

                if char == '.' {
                    continue;
                }

                if let Some(digit) = char.to_digit(10) {
                    part_value = part_value * 10 + digit;
                    reading_digits = true;
                    continue;
                }

                if char == '*' {
                    self.gears.push((x as u32, y as u32));
                }

                self.symbols.push((x as u32, y as u32));
            }

            if reading_digits {
                let width = (part_value.ilog10() + 1) as usize;
                let part_x = (line.len() - 1) as u32 - width as u32;

                let part = PartNumber {
                    value: part_value,
                    width,
                    x: part_x,
                    y: y as u32,
                };

                self.part_numbers.push(part);
            }
        }

        Ok(())
    }

    fn part_1(&self) -> anyhow::Result<String> {
        let mut sum = 0;

        'outer: for part_number in &self.part_numbers {
            for symbol in &self.symbols {
                if part_number.adjacent(symbol.0, symbol.1) {
                    sum += part_number.value;
                    continue 'outer;
                }
            }
        }

        Ok(sum.to_string())
    }

    fn part_2(&self) -> anyhow::Result<String> {
        let mut gear_ratio_sum = 0;

        for (gx, gy) in &self.gears {
            let adjacent: Vec<u32> = self
                .part_numbers
                .iter()
                .filter_map(|part| {
                    if part.adjacent(*gx, *gy) {
                        Some(part.value)
                    } else {
                        None
                    }
                })
                .take(3)
                .collect();

            if adjacent.len() == 2 {
                gear_ratio_sum += adjacent[0] * adjacent[1];
            }
        }

        Ok(gear_ratio_sum.to_string())
    }
}
