use std::str::FromStr;

use crate::constants;
use crate::{Operation, operations::{OperationType, Segment, ARITHMETIC_OPERATION, BRANCHING_OPERATION, MEMORY_OPERATION, FUNCTION_OPERATION}};

pub struct Command {
    /// Command number of current for loops
    command_count: u32,
    /// Name of the program being executed (used by static and stuff)
    program_name: String,
    /// If it has a valid command or if it is blank
    is_valid: bool,
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
    /// Location index or value of the segment being written to
    segment_i: u32,
    /// Resulting command strings (assembly commands) after the original command is processed
    parsed_cmd: Vec<String>,

}

impl Command {
    pub fn new(command_str: &str, command_cnt: u32, prgm_name: &str) -> Self {
        let mut command = Command {
            command_count: command_cnt,
            program_name: prgm_name.to_owned(),
            is_valid: false,
            command_string: command_str.to_owned(),
            command_tokens: command_str.split_whitespace().map(str::to_string).collect(),
            operation: Operation::Default,
            operation_type: OperationType::Default,
            segment: Segment::Default,
            segment_i: 0,
            parsed_cmd: Vec::new()
        };

        command.is_valid = command.parse();

        return command;
    }

    pub fn get_processed(&self) -> Option<&Vec<String>> {
        if !self.is_valid {
            return None;
        }
        return Some(&self.parsed_cmd);
    }

    pub fn has_command(&self) -> bool {
        return self.is_valid;
    }

    fn append_cmd(&mut self, cmd: &str) {
        self.parsed_cmd.push(cmd.to_owned());
    }

    /// Set d register to value i
    fn set_d(&mut self, i: u32) {
        self.append_cmd(&format!("@{}", i));
        self.append_cmd("D=A");
    }

    /// Push whatever is in d onto the stack
    fn push_d(&mut self) {
        self.append_cmd("@SP");
        self.append_cmd("A=M");
        self.append_cmd("M=D");
        // Increment SP
        self.inc_sp();
    }

    /// Pop whatever is on the stack to d
    fn pop_d(&mut self) {
        self.append_cmd("@SP");
        self.append_cmd("A=M-1");
        self.append_cmd("D=M");
        // Decrement
        self.dec_sp();
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

    /// Decrement the stack pointer
    fn dec_sp(&mut self) {
        // Decrement SP
        self.append_cmd("@SP");
        self.append_cmd("M=M-1");
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
    fn parse(&mut self) -> bool {
        // Check if it's a comment
        if self.command_string.find("//") == Some(0) || self.command_string == "" {
            return false;
        }

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
        
        return true;

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
                // self.inc_sp();
            },
            Operation::Gt   => {
                self.get_sp1();
                self.get_sp2();
                self.append_cmd("D=M-D"); // We want x - y which is M - D
                self.load_bool_jumps("JGT"); // We want true if gt
                // self.inc_sp();
            },
            Operation::Lt   => {
                self.get_sp1();
                self.get_sp2();
                self.append_cmd("D=M-D");
                self.load_bool_jumps("JLT"); // We want true if lt
                // self.inc_sp();
            },
            Operation::And  => {
                self.get_sp1();
                self.get_sp2();
                self.append_cmd("D=M&D");
                self.push_d();
                // self.inc_sp();
            },
            Operation::Or   => {
                self.get_sp1();
                self.get_sp2();
                self.append_cmd("D=M|D");
                self.push_d();
                // self.inc_sp();
            }
            _               => {}
        }

    }

    /// Parse branching command into its hack commands
    fn parse_branching(&mut self) {

    }

    /// Parse memory command into its hack commands
    fn parse_memory(&mut self) {
        // Assign segment
        self.segment = Segment::from_str(&self.command_tokens[1]).unwrap();

        // Assign segment index
        self.segment_i = self.command_tokens[2].parse().unwrap();
        
        // Save memory location as a string
        let mut memory_addr = Segment::to_string(&self.segment);

        // Parse local, argument, this, that
        if self.segment == Segment::Local || self.segment == Segment::Argument
            || self.segment == Segment::This || self.segment == Segment::That {

            if self.operation == Operation::Push {
                // Get the offset in the d register
                self.set_d(self.segment_i);
                // Get to the new memory address and add in the offset
                self.append_cmd(&format!("@{}", memory_addr));
                self.append_cmd("A=D+M"); // Go to the address

                // Get value at ram in d reg
                self.append_cmd("D=M");
                // Now push d
                self.push_d();
            }
            else if self.operation == Operation::Pop {
                // Get the offset in d
                self.set_d(self.segment_i);
                
                // Now go to the base and get address (base + i) in d
                self.append_cmd(&format!("@{}", memory_addr));
                self.append_cmd("D=M+D");

                // Now save d in R13
                self.append_cmd("@R13");
                self.append_cmd("M=D");

                // Get SP value into d
                self.pop_d();

                // Go to R13 and follow the pointer
                self.append_cmd("@R13");
                self.append_cmd("A=M");

                // Now Save D into M
                self.append_cmd("M=D");

            }
            else {
                // Should never get here
            }
        }
        else if self.segment == Segment::Static {
            if self.operation == Operation::Push {
                // Go to memory location
                self.append_cmd(&format!("@{}.{}", self.program_name, self.segment_i));
                // Get the value in d
                self.append_cmd("D=M");
                // Push d to the stack
                self.push_d();
            }
            else if self.operation == Operation::Pop {
                // Pop d
                self.pop_d();
                // Go to memory location
                self.append_cmd(&format!("@{}.{}", self.program_name, self.segment_i));
                // Set M to d
                self.append_cmd("M=D");
            }
            else {
                // Should never get here
            }
        }
        else if self.segment == Segment::Temp {
            let addr = constants::TEMP_START + self.segment_i;

            if self.operation == Operation::Push {
                // Go to address
                self.append_cmd(&format!("@{}", addr));
                // Set d to m
                self.append_cmd("D=M");
                // Push d
                self.push_d();        
            }
            else if self.operation == Operation::Pop {
                // Pop d
                self.pop_d();
                // Go to address
                self.append_cmd(&format!("@{}", addr));
                // Set m to d
                self.append_cmd("M=D");
            }
            else {
                // Should never get here
            }
        }
        else if self.segment == Segment::Pointer {
            // Get corresponding segment
            memory_addr = if self.segment_i == 0 {"THIS".to_owned()} else {"THAT".to_owned()};

            if self.operation == Operation::Push {
                // Go to either this or that
                self.append_cmd(&format!("@{}", memory_addr));
                // Store address on stack
                // ISSUE ? This might need to be M instead of D
                self.append_cmd("D=M");
                self.push_d();
            }
            else if self.operation == Operation::Pop {
                // Pop D
                self.pop_d();
                // Go to this or that and store value into it from stack
                self.append_cmd(&format!("@{}", memory_addr));
                self.append_cmd("M=D");
            }
            else {
                // Should never get here
            }
        }
        else if self.segment == Segment::Constant {
            if self.operation == Operation::Push {
                // Get the constant value
                self.set_d(self.segment_i);
                self.push_d();
            }
            else if self.operation == Operation::Pop {
                // Should never get here
            }
            else {
                // Should never get here
            }
        }
        
    }

    /// Parse function command into its hack commands
    fn parse_function(&mut self) {

    }
}