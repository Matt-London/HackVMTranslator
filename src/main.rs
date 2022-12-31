mod operations;
mod parser;

use crate::operations::*;
use crate::parser::{Command, Parser};

fn main() {
    println!("Hello, world!");

    if FUNCTION_OPERATION.contains(Operation::Function) {
        println!("Function in function op");
    }

    if !FUNCTION_OPERATION.contains(Operation::Add) {
        println!("Add in function not in op");
    }
}
