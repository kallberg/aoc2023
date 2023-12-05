use crate::Solver;
use anyhow::Result;
use std::str::Lines;

#[derive(Default)]
struct Map(pub Vec<(usize, usize, usize)>);

impl Map {
    fn add_mapping(&mut self, dst_start: usize, src_start: usize, length: usize) {
        self.0.push((dst_start, src_start, length));
    }

    fn lookup(&self, src: usize) -> usize {
        for (dst_start, src_start, length) in self.0.clone() {
            let src_range = src_start..(src_start + length);
            let mut dst_range = dst_start..(dst_start + length);

            if src_range.contains(&src) {
                let index = src - src_range.start;

                let dst = dst_range.nth(index).unwrap();

                return dst;
            }
        }

        src
    }
}

#[derive(Default)]
pub struct Day {
    input: String,
    seeds: Vec<usize>,
    seed: Map,
    soil: Map,
    fertilizer: Map,
    water: Map,
    light: Map,
    temperature: Map,
    humidity: Map,
}

impl Day {
    fn location(&self, seed: usize) -> usize {
        let soil = self.seed.lookup(seed);
        let fertilizer = self.soil.lookup(soil);
        let water = self.fertilizer.lookup(fertilizer);
        let light = self.water.lookup(water);
        let temperature = self.light.lookup(light);
        let humidity = self.temperature.lookup(temperature);
        let location = self.humidity.lookup(humidity);

        location
    }
}

impl Solver for Day {
    fn setup(&mut self, input: &str) {
        self.input = input.to_string();
    }

    fn parse(&mut self) -> Result<()> {
        let mut lines = self.input.lines();

        let seed_line = lines.next().expect("seed line");
        let seed_line = seed_line.split_once(": ").unwrap().1;
        self.seeds = seed_line
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        fn parse_map(lines: &mut Lines, map: &mut Map) {
            let mut line = lines.next().unwrap();

            while !line.is_empty() {
                let (left, right) = line.split_once(" ").unwrap();
                let dst_start = left.parse().unwrap();
                let (left, right) = right.split_once(" ").unwrap();
                let src_start = left.parse().unwrap();
                let length = right.parse().unwrap();

                map.add_mapping(dst_start, src_start, length);
                line = lines.next().unwrap_or_default();
            }

            lines.next();
        }

        lines.next();
        lines.next();

        parse_map(&mut lines, &mut self.seed);
        parse_map(&mut lines, &mut self.soil);
        parse_map(&mut lines, &mut self.fertilizer);
        parse_map(&mut lines, &mut self.water);
        parse_map(&mut lines, &mut self.light);
        parse_map(&mut lines, &mut self.temperature);
        parse_map(&mut lines, &mut self.humidity);

        Ok(())
    }

    fn part_1(&self) -> Result<String> {
        let mut locations = vec![];

        for seed in &self.seeds {
            locations.push(self.location(*seed));
        }

        Ok(locations.iter().min().unwrap().to_string())
    }

    fn part_2(&self) -> Result<String> {
        Ok(String::from("Placeholder"))
    }
}
