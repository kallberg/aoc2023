use crate::Solver;

type GameSet = (u32, u32, u32);

pub struct Day {
    rgb_game_max: Vec<(u32, u32, u32)>,
}

impl Day {
    fn parse_game_set(value: &str) -> GameSet {
        let (amount_str, color) = value.split_once(" ").unwrap();

        let amount: u32 = amount_str.parse().unwrap();

        match color {
            "blue" => (0, 0, amount),
            "red" => (amount, 0, 0),
            "green" => (0, amount, 0),
            _ => unreachable!(),
        }
    }

    fn line_game_set(set: &str) -> Vec<GameSet> {
        let mut reveals = vec![];
        let cubes = set.split(",");

        for cube in cubes {
            reveals.push(Day::parse_game_set(cube.trim()));
        }

        reveals
    }
    fn line_reveals(line: &str) -> Vec<GameSet> {
        let line = line.split_once(":").unwrap().1;
        let sets = line.split(";");

        let mut game_sets = vec![];

        for set in sets {
            game_sets.append(&mut Day::line_game_set(set))
        }

        game_sets
    }

    fn games(input: &str) -> Vec<Vec<GameSet>> {
        let mut lines_reveals = vec![];

        for line in input.lines() {
            let line_reveals = Day::line_reveals(line);

            lines_reveals.push(line_reveals);
        }

        lines_reveals
    }

    fn game_max_rgb(line: Vec<GameSet>) -> (u32, u32, u32) {
        line.into_iter()
            .reduce(|(ar, ag, ab), (r, g, b)| (ar.max(r), ag.max(g), ab.max(b)))
            .unwrap_or((0, 0, 0))
    }
}

impl From<&str> for Day {
    fn from(value: &str) -> Self {
        let games = Day::games(value);

        let mut rgb_game_max = vec![];

        for game in games.into_iter() {
            rgb_game_max.push(Day::game_max_rgb(game))
        }

        Self { rgb_game_max }
    }
}

impl Solver for Day {
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
