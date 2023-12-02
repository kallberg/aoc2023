use crate::Solver;

pub enum Reveal {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl From<&str> for Reveal {
    fn from(value: &str) -> Self {
        let (amount_str, color) = value.split_once(" ").unwrap();

        let amount: u32 = amount_str.parse().unwrap();

        match color {
            "blue" => Reveal::Blue(amount),
            "red" => Reveal::Red(amount),
            "green" => Reveal::Green(amount),
            _ => unreachable!(),
        }
    }
}

pub struct Day {
    input: String,
}

impl From<&str> for Day {
    fn from(value: &str) -> Self {
        Self {
            input: value.to_string(),
        }
    }
}

impl Solver for Day {
    fn part_1(&self) -> anyhow::Result<String> {
        let mut id_sum = 0;

        for (index, line) in self.input.lines().enumerate() {
            let id = index + 1;
            let line = line.split_once(":").unwrap().1;
            let sets = line.split(";");

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            for s in sets {
                let cubes = s.split(",");

                for cube in cubes {
                    let reveal = Reveal::from(cube.trim());
                    match reveal {
                        Reveal::Red(amount) => {
                            red += amount;
                            max_red = max_red.max(amount);
                        }
                        Reveal::Green(amount) => {
                            green += amount;
                            max_green = max_green.max(amount);
                        }
                        Reveal::Blue(amount) => {
                            blue += amount;
                            max_blue = max_blue.max(amount);
                        }
                    }
                }
            }

            let power = max_red * max_green * max_blue;

            if max_red > 12 || max_green > 13 || max_blue > 14 {
                continue;
            }

            println!("{}", id);

            id_sum += id;
        }

        Ok(id_sum.to_string())
    }

    fn part_2(&self) -> anyhow::Result<String> {
        let mut power_sum = 0;

        for line in self.input.lines() {
            let line = line.split_once(":").unwrap().1;
            let sets = line.split(";");

            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            for s in sets {
                let cubes = s.split(",");

                for cube in cubes {
                    let reveal = Reveal::from(cube.trim());
                    match reveal {
                        Reveal::Red(amount) => {
                            max_red = max_red.max(amount);
                        }
                        Reveal::Green(amount) => {
                            max_green = max_green.max(amount);
                        }
                        Reveal::Blue(amount) => {
                            max_blue = max_blue.max(amount);
                        }
                    }
                }
            }

            power_sum += max_red * max_green * max_blue;
        }

        Ok(power_sum.to_string())
    }
}
