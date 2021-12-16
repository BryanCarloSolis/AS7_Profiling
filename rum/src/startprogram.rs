// the start_program module interface holds the function that runs the program 
// it loops through and gets the instructions that are read from the inputted file and
// matches the opcode to the Uvm instruction and computes specified array of instructions  

use crate::uvm::Uvm;
pub mod instr;

use instr::Opcode;

use std::{process};

// this program begins running the program and all of its instructions
// runs the Uvm until it halts
pub fn start_program(mut machine: Uvm, mut program_counter: usize) -> (){
    loop { 
        // machine gets and stores array of instructions
        let instruction = machine.get_instruction(program_counter);

        //iterates through loop
        program_counter = program_counter + 1;

        // matches opcode to Uvm instruction and runs the specified instruction
        match instruction.opcode {
            Opcode::CMov => machine.cmov(instruction),
            Opcode::Load => machine.load(instruction),
            Opcode::Store => machine.store(instruction),
            Opcode::Add => machine.add(instruction),
            Opcode::Mul => machine.mul(instruction),
            Opcode::Div => machine.div(instruction),
            Opcode::Nand => machine.nand(instruction),
            Opcode::Halt => process::exit(0),
            Opcode::MapSegment => machine.map_segment(instruction),
            Opcode::UnmapSegment => machine.unmap_segment(instruction),
            Opcode::Output => machine.output(instruction),
            Opcode::Input => machine.input(instruction),
            Opcode::LoadProgram => program_counter = machine.load_program(instruction),
            Opcode::LoadValue => machine.load_value(instruction),
            Opcode::Error => panic!(
                "Opcode not recognized"
            )
        }

    }



}
