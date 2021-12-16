
/*
Compile Instructions:
1. cargo check
2. cargo build --release
3. target/release/rum midmark.um

*/

// the main.rs module interface serves to get the command line arguments from stdin and 
// call the start_program to run it and all its instructions


use rum::startprogram::start_program;
use rum::uvm::Uvm;
use rum::read_instr::read_instrs;


fn main() {
    // takes in arguments from stdin - argument is single file
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    // takes in the instructions from the file and sets the machine to store the instructions
    // pc 
    let instructions = read_instrs(filename);
    let machine = Uvm::new(instructions);
    let program_counter : usize = 0;

    // calls start_program function to run the program and array of all the instructions 
    start_program(machine, program_counter);

}
