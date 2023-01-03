mod constants;
mod operations;
mod parser;

use crate::operations::*;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Input file path
    input_path: String,
    /// Output file path
    output_path: Option<String>,
}

fn main() {
    let args = Cli::parse();

    let mut parser = parser::Parser::new(&args.input_path);

    // Get the default name (input_path with .asm)
    let default_name = args.input_path.replace(".vm", ".asm");

    match args.output_path {
        Some(output) => parser.output(&output),
        None => parser.output(&default_name)
    }
    
}
