use crate::Solver;
use anyhow::Result;

fn ways(time: f64, distance: f64) -> u64 {
    let root = (time.powi(2) - 4.0 * distance).sqrt();

    let x1 = (-time + root) / -2.0;
    let x2 = (-time - root) / -2.0;

    (x2.ceil() - x1.floor()) as u64 - 1
}

pub struct Race {
    time: u64,
    record_distance: u64,
}

#[derive(Default)]
pub struct Day {
    input: String,
    races: Vec<Race>,
}

impl Solver for Day {
    fn setup(&mut self, input: &str) {
        self.input = input.to_string();
    }

    fn parse(&mut self) -> Result<()> {
        let mut lines = self.input.lines();
        let times = lines.next().unwrap();
        let distances = lines.next().unwrap();

        let times = times.split_once(":").unwrap().1;
        let times = times
            .split_whitespace()
            .map(|time_str| time_str.parse::<u64>());
        let distances = distances.split_once(":").unwrap().1;
        let distances = distances
            .split_whitespace()
            .map(|distance_str| distance_str.parse::<u64>());

        for (time, distance) in times.zip(distances) {
            self.races.push(Race {
                time: time?,
                record_distance: distance?,
            });
        }

        Ok(())
    }

    fn part_1(&self) -> Result<String> {
        let mut output = vec![];

        for race in &self.races {
            output.push(ways(race.time as f64, race.record_distance as f64))
        }

        Ok(output.iter().product::<u64>().to_string())
    }

    fn part_2(&self) -> Result<String> {
        let mut record_time: u64 = 0;
        let mut record_distance: u64 = 0;

        for race in self.races.iter().rev() {
            let time_numbers = if record_time > 0 {
                record_time.ilog10() + 1
            } else {
                0
            };
            let distance_numbers = if record_distance > 0 {
                record_distance.ilog10() + 1
            } else {
                0
            };
            record_time += race.time * 10u64.pow(time_numbers);
            record_distance += race.record_distance as u64 * 10u64.pow(distance_numbers);
        }

        let ways = ways(record_time as f64, record_distance as f64);

        Ok(ways.to_string())
    }
}
