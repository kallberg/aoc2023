use crate::Solver;
use anyhow::Result;
use std::ops::Range;
use std::str::Lines;

fn intersects(output_range: &Range<usize>, input_range: &Range<usize>) -> bool {
    output_range.start <= input_range.end && output_range.end > input_range.start
}

struct Mapping {
    range: Range<usize>,
    delta: isize,
}

#[derive(Default)]
struct Map {
    mappings: Vec<Mapping>,
}

impl Map {
    fn add_mapping(&mut self, dst_start: usize, src_start: usize, length: usize) {
        self.mappings.push(Mapping {
            range: src_start..(src_start + length),
            delta: dst_start as isize - src_start as isize,
        });
    }

    fn map_one(&self, src: usize) -> usize {
        for mapping in &self.mappings {
            if mapping.range.contains(&src) {
                return (src as isize + mapping.delta) as usize;
            }
        }

        src
    }

    fn map_range(&self, range: Range<usize>) -> Vec<Range<usize>> {
        let mut output = vec![];

        let intersecting_mappings = self
            .mappings
            .iter()
            .filter(|mapping| intersects(&mapping.range, &range));

        let mut start = range.start;

        for mapping in intersecting_mappings {
            if start < mapping.range.start {
                output.push(start..mapping.range.start);
                start = mapping.range.end;
            }

            let mapped_start = ((start as isize) + mapping.delta) as usize;
            let range_mapped_end = (range.end as isize + mapping.delta) as usize;
            let mapping_mapped_end = (mapping.range.end as isize + mapping.delta) as usize;
            let mapped_end = mapping_mapped_end.min(range_mapped_end);

            output.push(mapped_start..mapped_end);

            start = range.end.min(mapping.range.end);
        }

        if start < range.end {
            output.push(start..range.end)
        }

        output
    }

    fn map_ranges(&self, inputs: Vec<Range<usize>>) -> Vec<Range<usize>> {
        let mut output = vec![];

        for input in inputs {
            let mut mapped = self.map_range(input);

            mapped.sort_by(|a, b| a.start.cmp(&b.start));

            for range in mapped {
                output.push(range);
            }
        }

        output
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
        let soil = self.seed.map_one(seed);
        let fertilizer = self.soil.map_one(soil);
        let water = self.fertilizer.map_one(fertilizer);
        let light = self.water.map_one(water);
        let temperature = self.light.map_one(light);
        let humidity = self.temperature.map_one(temperature);
        let location = self.humidity.map_one(humidity);

        location
    }

    fn location_ranges(&self, seed_ranges: Vec<Range<usize>>) -> Vec<Range<usize>> {
        let soil = self.seed.map_ranges(seed_ranges);
        let fertilizer = self.soil.map_ranges(soil);
        let water = self.fertilizer.map_ranges(fertilizer);
        let light = self.water.map_ranges(water);
        let temperature = self.light.map_ranges(light);
        let humidity = self.temperature.map_ranges(temperature);
        let location = self.humidity.map_ranges(humidity);

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

            map.mappings
                .sort_by(|a, b| a.range.start.cmp(&b.range.start));

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
        let seed_ranges: Vec<Range<usize>> = self
            .seeds
            .chunks_exact(2)
            .map(|chunks| chunks[0]..(chunks[0] + chunks[1]))
            .collect();

        let min = self
            .location_ranges(seed_ranges)
            .iter()
            .map(|range| range.start)
            .min()
            .unwrap_or(usize::MAX);

        Ok(min.to_string())
    }
}
