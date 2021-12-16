// the read_instr module interface reads the instructions from the file
// they are read and stored as a sequence of unsigned 32 bit words with big-endian byte order

use byteorder::{ReadBytesExt, BigEndian};
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Cursor;

// funct to read instructions from file and stores it on the heap
pub fn read_instrs(file: &str) -> Vec<u32> {
    let mut temp = File::open(file).expect("Unable to open");

    // initializes data and instructions as mutable vectors
    let mut data = Vec::new();
    let mut instructions = Vec::new();

    // reads and matches to then store data onto the heap in big-endian byte order
    match temp.read_to_end(&mut data) {
        Ok(_bytes) => {
            
            // loops through length of data 
            for i in 0..(data.len()/4) {
                let index_start = i * 4;
                let index_end = index_start + 4;

                let buffer = &data[index_start..index_end];

                // reads the instructions as u32 bit words in Big Endian order
                let instruction = Cursor::new(buffer).read_u32::<BigEndian>().unwrap();

                //pushes or stores instructions from file
                instructions.push(instruction);
            }
            instructions
        },
        Err(_e) => panic!()
    }
}