use crate::Solver;
use anyhow::Result;
use std::collections::HashMap;
use thiserror::Error;

pub struct Node {
    left: String,
    right: String,
}

pub enum Instruction {
    Left,
    Right,
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

            let node = Node { left, right };

            self.nodes.insert(id, node);
        }

        Ok(())
    }

    fn part_1(&self) -> Result<String> {
        let mut instructions = self.instructions.iter().cycle();

        let mut current_node = "AAA";
        let mut steps = 0;

        loop {
            let instruction = instructions.next().unwrap();
            let node = self.nodes.get(current_node).ok_or(DayError::MissingNode)?;

            match instruction {
                Instruction::Left => current_node = &node.left,
                Instruction::Right => current_node = &node.right,
            }
            steps += 1;

            if current_node == "ZZZ" {
                break;
            }
        }

        Ok(steps.to_string())
    }

    fn part_2(&self) -> Result<String> {
        Ok("Placeholder".into())
    }
}
