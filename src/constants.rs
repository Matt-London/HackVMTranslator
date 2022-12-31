use enumset::{EnumSetType, EnumSet, enum_set};

/// Operation being performed by a command
#[derive(EnumSetType, Debug)]
pub enum Operation {
    /// No operation
    Noop,
    /// Arithmetic operations
    Add,
    Sub,
    Neg,
    Eq,
    Get,
    Lt,
    And,
    Or,
    Not,
    /// Branching operations
    Label,
    Goto,
    IfGoto,
    /// Memory operations
    Push,
    Pop,
    /// Function operations
    Function,
    Call,
    Return
}

/// Type of operation to be performed
#[derive(EnumSetType, Debug)]
pub enum OperationType {
    Arithmetic,
    Branching,
    Memory,
    Function
}

/// The different memory segments supported
#[derive(EnumSetType, Debug)]
pub enum Segment {
    None,
    Sp,
    Local,
    Argument,
    This,
    That,
    Constant,
    Static,
    Pointer,
    Temp
}

/// Set of arithmetic operations
pub const ARITHMETIC_OPERATION: EnumSet<Operation> = enum_set!(
    Operation::Add |
    Operation::Sub |
    Operation::Neg |
    Operation::Eq |
    Operation::Get |
    Operation::Lt |
    Operation::And |
    Operation::Or |
    Operation::Not
);

/// Set of branching operations
pub const BRANCHING_OPERATION: EnumSet<Operation> = enum_set!(
    Operation::Label |
    Operation::Goto |
    Operation::IfGoto
);

/// Set of memory operations
pub const MEMORY_OPERATION: EnumSet<Operation> = enum_set!(
    Operation::Push |
    Operation::Pop
);

/// Set of function operations
pub const FUNCTION_OPERATION: EnumSet<Operation> = enum_set!(
    Operation::Function |
    Operation::Call |
    Operation::Return
);