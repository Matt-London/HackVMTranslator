use crate::{Operation, operations::{OperationType, Segment}};

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
    pub fn new(commandString: &str) -> Self {
        let mut command = Command {
            command_string: commandString.to_owned(),
            command_tokens: commandString.split_whitespace().map(str::to_string).collect(),
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