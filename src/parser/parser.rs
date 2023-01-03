use crate::parser::Command;

use std::fs::File;
use std::path::Path;
use std::io::{prelude::*, BufReader};

use substring::Substring;

pub struct Parser {
    vm_path: String,
    command_list: Vec<Command>
}

impl Parser {
    pub fn new(fname: &str) -> Self {
        let mut parser = Parser {
            vm_path: fname.to_owned(),
            command_list: Vec::new()
        };

        parser.process_cmds();

        return parser;
    }

    fn process_cmds(&mut self) {
        // Get base file name
        let path = Path::new(&self.vm_path);
        let full_name = path.file_name().unwrap().to_os_string().into_string().unwrap();

        let file_name = full_name.substring(0, full_name.find(".").unwrap());

        let vm_code = File::open(&self.vm_path).unwrap();
        let bf = BufReader::new(vm_code);

        let mut cmd_cnt = 0;
        for line  in bf.lines() {
            let command = Command::new(&line.unwrap(), cmd_cnt, file_name);

            if command.has_command() {
                self.command_list.push(command);
                cmd_cnt += 1;
            }
        }

    }

    pub fn output(&mut self, output_path: &str) {
        // Open file for outputting
        let mut asm_code = File::create(output_path).expect("Failed to open file");

        // Loop through processed commands
        for cmd in self.command_list.iter() {
            // Loop through each string in cmd
            for asm_cmd in cmd.get_processed().unwrap().iter() {
                asm_code.write_all(format!("{}\n", asm_cmd).as_bytes()).expect("Failed to write");
            }
        }
        // match bw.flush() {
        //     Ok(_)   => (),
        //     Err(_)  => println!("Failed to flush to file")
        // }

    }
}