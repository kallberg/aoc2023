use crate::Solver;

#[derive(Default)]
pub struct Day {
    input: String,
}

impl Solver for Day {
    fn setup(&mut self, input: &str) {
        self.input = input.to_string();
    }

    fn parse(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn part_1(&self) -> anyhow::Result<String> {
        Ok("Placeholder".into())
    }

    fn part_2(&self) -> anyhow::Result<String> {
        Ok("Placeholder".into())
    }
}
