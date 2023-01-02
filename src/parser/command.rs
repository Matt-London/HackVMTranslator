use std::fmt::format;
use std::str::FromStr;

use crate::constants;
use crate::{Operation, operations::{OperationType, Segment, ARITHMETIC_OPERATION, BRANCHING_OPERATION, MEMORY_OPERATION, FUNCTION_OPERATION}};

pub struct Command {
    /// Command number of current for loops
    command_count: u32,
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
    pub fn new(command_str: &str, command_cnt: u32) -> Self {
        let mut command = Command {
            command_count: command_cnt,
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

    fn append_cmd(&mut self, cmd: &str) {
        self.parsed_cmd.push(cmd.to_owned());
    }

    /// Save SP1 in current
    fn get_sp1(&mut self) {
        self.append_cmd("@SP");     // Go to stack pointer
        self.append_cmd("AM=M-1");  // Get pointer at SP - 1 and goto address
    }

    /// Save SP1 in D and locate at SP2
    fn get_sp2(&mut self) {
        self.append_cmd("D=M");     // Save the value at the address in D reg
        self.append_cmd("@SP");     // Go back to the stack pointer
        // ISSUE: This may cause issues if M is saved after A
        self.append_cmd("AM=M-1");  // Go back to SP - 1 (second val) and jump to value
        // Next line will be something like M=M+D depending on operation
    }

    /// Increment the stack pointer
    fn inc_sp(&mut self) {
        // Increment SP
        self.append_cmd("@SP");
        self.append_cmd("M=M+1");
    }

    /// Run this function after loading operating value into D and decider as a param
    ///
    /// Example
    ///     jump_ins = "JEQ"
    /// 
    ///     Now this function will write the behavior that if D is EQ 0 it will
    ///     load true val on the stack, otherwise false val
    fn load_bool_jumps(&mut self, jump_ins: &str) {
        let true_label = "RESULT_TRUE";
        let false_label = "RESULT_FALSE";
        let set_val_label = "RESULT_SET";

        self.append_cmd(&format!("@{}_{}", true_label, self.command_count)); // Set the true label
        self.append_cmd(&format!("D;{}", jump_ins)); // Jump if D is 0
        self.append_cmd(&format!("@{}_{}", false_label, self.command_count)); // Set false label
        self.append_cmd("0;JMP"); // Jump regardless
        // Now setup the labels
        // True label
        self.append_cmd(&format!("({}_{})", true_label, self.command_count));
        self.append_cmd(&format!("D={}", constants::TRUE_VALUE)); // Set the true value in D
        self.append_cmd(&format!("@{}_{}", set_val_label, self.command_count)); // Load in set label
        self.append_cmd("0;JMP"); // Jump to that label

        // False label
        self.append_cmd(&format!("({}_{})", false_label, self.command_count));
        self.append_cmd(&format!("D={}", constants::FALSE_VALUE)); // Set the false value in D
        // Let it fall through into the result label

        // Set value label
        self.append_cmd(&format!("({}_{})", set_val_label, self.command_count));
        self.append_cmd("@SP");
        self.append_cmd("A=M"); // Go to SP
        self.append_cmd("M=D"); // Set *SP to the result val
        // Now increment sp
        self.inc_sp();


    }

    /// Determines the correct parse call for the given operation type
    fn parse(&mut self) {
        // Clear and append header of operation (original content)
        self.parsed_cmd.clear();
        self.append_cmd(&format!("// {}", self.command_string));

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
            OperationType::Default => {}

        }
        

    }

    /// Parse arithmetic command into its hack commands
    fn parse_arithmetic(&mut self) {
        // Check if it is not or neg as both of those only take one argument
        if self.operation == Operation::Neg {
            self.get_sp1();
            self.append_cmd("M=-M"); // Make M negative and save
            self.inc_sp();
        }
        else if self.operation == Operation::Not {
            self.get_sp1();
            self.append_cmd("M=!M"); // Negate M
            self.inc_sp();
        }

        // Match on operation
        match self.operation {
            Operation::Add  => {
                self.get_sp1();
                self.get_sp2();
                self.append_cmd("M=M+D"); // Insert the new value at SP
                self.inc_sp();
            },
            Operation::Sub  => {
                self.get_sp1();
                self.get_sp2();
                self.append_cmd("M=M-D");
                self.inc_sp();
            },
            Operation::Eq   => {
                self.get_sp1();
                self.get_sp2();
                self.append_cmd("D=M-D"); // If equal this value is 0
                self.load_bool_jumps("JEQ"); // We want true if eq
                self.inc_sp();
            },
            Operation::Gt   => {
                self.get_sp1();
                self.get_sp2();
                self.append_cmd("D=M-D"); // We want x - y which is M - D
                self.load_bool_jumps("JGT"); // We want true if gt
                self.inc_sp();
            },
            Operation::Lt   => {
                self.get_sp1();
                self.get_sp2();
                self.append_cmd("D=M-D");
                self.load_bool_jumps("JLT"); // We want true if lt
                self.inc_sp();
            },
            Operation::And  => {
                self.get_sp1();
                self.get_sp2();
                self.append_cmd("D=M&D");
                self.append_cmd("D=D-1"); // Now we subtract 1. If true we have 0 in D
                self.load_bool_jumps("JEQ"); // We want true if eq
                self.inc_sp();
            },
            Operation::Or   => {
                self.get_sp1();
                self.get_sp2();
                self.append_cmd("D=M|D");
                self.append_cmd("D=D-1"); // Now we subtract 1. If true we have 0 in D
                self.load_bool_jumps("JEQ"); // We want true if eq
                self.inc_sp();
            }
            _               => {}
        }

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