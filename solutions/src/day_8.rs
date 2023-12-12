use crate::Solver;
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use thiserror::Error;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: &usize) -> usize {
    a * b / gcd(a, *b)
}

pub struct Node {
    id: String,
    left: String,
    right: String,
    ghost_start: bool,
    ghost_end: bool,
}

pub enum Instruction {
    Left,
    Right,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Left => 'L',
            Instruction::Right => 'R',
        }
        .fmt(f)
    }
}

#[derive(Error, Debug)]
pub enum DayError {
    #[error("parse of non-instruction character")]
    ParseInstruction,
    #[error("missing node assignment")]
    ParseNodeAssignment,
    #[error("missing left right delimiter")]
    ParseLeftRightDelimiter,
    #[error("missing node")]
    MissingNode,
}

impl TryFrom<char> for Instruction {
    type Error = DayError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => return Err(DayError::ParseInstruction),
        })
    }
}

#[derive(Default)]
pub struct Day {
    input: String,
    instructions: Vec<Instruction>,
    nodes: HashMap<String, Node>,
}

fn ghost_endpoints(
    id: &str,
    instructions: &Vec<Instruction>,
    nodes: &HashMap<String, Node>,
) -> Vec<usize> {
    let instruction_set_size = instructions.len();
    let mut node = nodes.get(id).unwrap();
    let mut endpoints = vec![];
    let mut seen: HashSet<(usize, &str)> = HashSet::new();

    seen.insert((0, id));

    let mut program_counter = 0;
    let mut instruction_index = program_counter % instruction_set_size;

    loop {
        let instruction = &instructions[instruction_index];

        match instruction {
            Instruction::Left => node = nodes.get(&node.left).unwrap(),
            Instruction::Right => node = nodes.get(&node.right).unwrap(),
        }

        if node.ghost_end {
            endpoints.push(program_counter);
        }

        program_counter += 1;
        instruction_index = program_counter % instruction_set_size;

        if seen.contains(&(instruction_index, &node.id)) {
            break;
        }

        seen.insert((instruction_index, &node.id));
    }
    endpoints
}

impl Solver for Day {
    fn setup(&mut self, input: &str) {
        self.input = input.to_string();
    }

    fn parse(&mut self) -> Result<()> {
        let mut lines = self.input.lines();

        for instruction_char in lines.next().unwrap().chars() {
            self.instructions
                .push(Instruction::try_from(instruction_char)?);
        }

        lines.next();

        for line in lines {
            let (id, rest) = line
                .split_once(" = ")
                .ok_or(DayError::ParseNodeAssignment)?;

            let end = rest.len() - 1;
            let rest = &rest[1..end];

            let (left, right) = rest
                .split_once(", ")
                .ok_or(DayError::ParseLeftRightDelimiter)?;

            let id = id.to_string();
            let left = left.to_string();
            let right = right.to_string();

            let node = Node {
                id: id.clone(),
                left,
                right,
                ghost_start: id.ends_with('A'),
                ghost_end: id.ends_with('Z'),
            };

            self.nodes.insert(id, node);
        }

        Ok(())
    }

    fn part_1(&self) -> Result<String> {
        let mut instructions = self.instructions.iter().cycle();

        let mut current_node = "AAA";
        let mut steps = 0;

        while current_node != "ZZZ" {
            let instruction = instructions.next().unwrap();
            let node = self.nodes.get(current_node).ok_or(DayError::MissingNode)?;

            match instruction {
                Instruction::Left => current_node = &node.left,
                Instruction::Right => current_node = &node.right,
            }
            steps += 1;
        }

        Ok(steps.to_string())
    }

    fn part_2(&self) -> Result<String> {
        let mut all_endpoints = vec![];

        for node in self.nodes.values() {
            if node.ghost_start {
                for index in ghost_endpoints(&node.id, &self.instructions, &self.nodes) {
                    all_endpoints.push(index + 1)
                }
            }
        }

        let lcm = all_endpoints.iter().fold(1usize, lcm);

        Ok(lcm.to_string())
    }
}
