use crate::Solver;

fn report_difference(report: &[i64]) -> Vec<i64> {
    report
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect()
}

fn report_sequence(report: &[i64], differences: &mut Vec<Vec<i64>>) {
    if report.iter().map(|v| v.abs()).sum::<i64>() == 0 {
        differences.push(report.into());
        return;
    }

    differences.push(report.into());

    let difference = report_difference(report);

    report_sequence(&difference, differences);
}

fn predict(report: &[i64]) -> i64 {
    let mut sequences = vec![];
    report_sequence(report, &mut sequences);

    let lasts = sequences
        .into_iter()
        .map(|sequence| *sequence.last().unwrap());

    let mut sum = 0;

    for last in lasts {
        sum += last;
    }

    sum
}

#[derive(Default)]
pub struct Day {
    input: String,
    reports: Vec<Vec<i64>>,
}

impl Solver for Day {
    fn setup(&mut self, input: &str) {
        self.input = input.to_string();
    }

    fn parse(&mut self) -> anyhow::Result<()> {
        for line in self.input.lines() {
            let mut report: Vec<i64> = vec![];
            for number_str in line.split_whitespace() {
                report.push(number_str.parse()?);
            }

            self.reports.push(report);
        }

        Ok(())
    }

    fn part_1(&self) -> anyhow::Result<String> {
        let mut all_sums = 0;

        for report in &self.reports {
            let sum = predict(&report);
            println!("{}", sum);
            all_sums += predict(report);
        }

        Ok(all_sums.to_string())
    }

    fn part_2(&self) -> anyhow::Result<String> {
        Ok("Placeholder".into())
    }
}
