use std::str::FromStr;

use crate::{Operation, operations::{OperationType, Segment, ARITHMETIC_OPERATION, BRANCHING_OPERATION, MEMORY_OPERATION, FUNCTION_OPERATION}};

pub struct Command {
    /// Original string being processed
    command_string: String,
    /// Original string split by whitespace
    command_tokens: Vec<String>,
    /// Operation being performed
    operation: Operation,
    /// Operation type being performed
    operation_type: OperationType,
    /// Memory segment being operated on
    segment: Segment,
    /// Location index of the segment being written to
    location_index: u32,
    /// Resulting command strings (assembly commands) after the original command is processed
    parsed_cmd: Vec<String>,

}

impl Command {
    pub fn new(command_str: &str) -> Self {
        let mut command = Command {
            command_string: command_str.to_owned(),
            command_tokens: command_str.split_whitespace().map(str::to_string).collect(),
            operation: Operation::Default,
            operation_type: OperationType::Default,
            segment: Segment::Default,
            location_index: 0,
            parsed_cmd: Vec::new()
        };

        command.parse();

        return command;
    }

    /// Determines the correct parse call for the given operation type
    fn parse(&mut self) {
        // Clear and append header of operation (original content)
        self.parsed_cmd.clear();
        self.parsed_cmd.push("// ".to_owned() + &self.command_string);

        let operation_str = &self.command_tokens[0];
        self.operation = Operation::from_str(operation_str).unwrap();

        // Figure out what type of operation
        if ARITHMETIC_OPERATION.contains(self.operation) {
            self.operation_type = OperationType::Arithmetic;
        }
        else if BRANCHING_OPERATION.contains(self.operation) {
            self.operation_type = OperationType::Branching;
        }
        else if MEMORY_OPERATION.contains(self.operation) {
            self.operation_type = OperationType::Memory;
        }
        else if FUNCTION_OPERATION.contains(self.operation) {
            self.operation_type = OperationType::Function;
        }
        else {
            self.operation_type = OperationType::Default;
        }

        // Now we will match on operation_type
        match self.operation_type {
            OperationType::Arithmetic => self.parse_arithmetic(),
            OperationType::Branching => self.parse_branching(),
            OperationType::Memory => self.parse_memory(),
            OperationType::Function => self.parse_function(),
            OperationType::Default => (),

        }

    }

    /// Parse arithmetic command into its hack commands
    fn parse_arithmetic(&mut self) {

    }

    /// Parse branching command into its hack commands
    fn parse_branching(&mut self) {

    }

    /// Parse memory command into its hack commands
    fn parse_memory(&mut self) {

    }

    /// Parse function command into its hack commands
    fn parse_function(&mut self) {

    }
}