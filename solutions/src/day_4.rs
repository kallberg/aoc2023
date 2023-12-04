use crate::Solver;

#[derive(Clone)]
struct Card {
    number: usize,
    winning: Vec<u32>,
    owned: Vec<u32>,
}

impl Card {
    fn matching_numbers(&self) -> u32 {
        let mut matches = 0;
        for number in &self.owned {
            if self.winning.contains(number) {
                matches += 1;
            }
        }
        return matches;
    }
}
#[derive(Default)]
pub struct Day {
    input: String,
    pile: Vec<Card>,
}

impl Solver for Day {
    fn setup(&mut self, input: &str) {
        self.input = input.to_string();
    }

    fn parse(&mut self) -> anyhow::Result<()> {
        for (index, line) in self.input.lines().enumerate() {
            let line = line.split_once(": ").unwrap().1;

            let (left, right) = line.split_once("|").unwrap();

            let mut winning = vec![];
            let mut owned = vec![];

            for number in left.trim().split_whitespace() {
                winning.push(number.parse()?);
            }

            for number in right.trim().split_whitespace() {
                owned.push(number.parse()?);
            }

            self.pile.push(Card {
                winning,
                owned,
                number: index + 1,
            });
        }

        Ok(())
    }

    fn part_1(&self) -> anyhow::Result<String> {
        let mut points: u32 = 0;

        for card in &self.pile {
            let matches = card.matching_numbers();

            if matches > 0 {
                points += 2u32.pow(matches - 1);
            }
        }

        Ok(points.to_string())
    }

    fn part_2(&self) -> anyhow::Result<String> {
        let mut counts = vec![];

        for _ in &self.pile {
            counts.push(1);
        }

        for card in &self.pile {
            let count = counts[card.number - 1];

            let mut matches = card.matching_numbers();

            while matches > 0 {
                let index = card.number - 1 + matches as usize;

                if index < counts.len() {
                    counts[index] += count;
                }

                matches -= 1;
            }
        }

        let sum: u32 = counts.iter().sum();

        Ok(sum.to_string())
    }
}
