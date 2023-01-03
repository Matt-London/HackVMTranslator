mod constants;
mod operations;
mod parser;

use crate::operations::*;
use crate::parser::Parser;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut parser = Parser::new(&args[1]);

    parser.output(&args[2]);
    
}
