mod seg;
mod reg;

const P_ADD: usize = 0;

use std::io::prelude::*;
use std::io::Read;
use std::io::{stdout, stdin};

use crate::uvm::seg::Segment;
use crate::uvm::reg::Regs;


use crate::startprogram::instr::Instr;

// Represents the structure of a UM machine
pub struct Uvm {
    seg: Segment,
    regs: Regs
}

impl Uvm {

    // Initializes a new machine
    pub fn new(instructions: Vec<u32>) -> Uvm{
        Uvm {
            seg: Segment::new(instructions),
            regs: Regs::new()
        }
    }

    // Gets the instruction given the program counter
    pub fn get_instruction(&self, pc: usize) -> Instr {
        self.seg.get_instruction(pc)
    }

    // Conditional Move
    pub fn cmov(&mut self, instruction: Instr) {
        

        if self.regs.get_reg(instruction.c.unwrap() as usize) != 0 {
            self.regs.set_reg(instruction.a as usize, self.regs.get_reg(instruction.b.unwrap() as usize));
        }
    }

    // Segmented Load
    pub fn load(&mut self, instruction: Instr) {
    

        let array = self.seg.get_seg(self.regs.get_reg(instruction.b.unwrap() as usize) as usize).unwrap();

        self.regs.set_reg(instruction.a as usize, array[self.regs.get_reg(instruction.c.unwrap() as usize) as usize]);
    }

    // Segmentted Store
    pub fn store(&mut self, instruction: Instr) {

        self.seg.set_seg(self.regs.get_reg(instruction.a as usize) as usize, 
        self.regs.get_reg(instruction.b.unwrap() as usize) as usize, 
        self.regs.get_reg(instruction.c.unwrap() as usize));
    }

    // Addition
    pub fn add(&mut self, instruction: Instr) {

        self.regs.set_reg(instruction.a as usize, 
            self.regs.get_reg(instruction.b.unwrap() as usize).wrapping_add(
                self.regs.get_reg(instruction.c.unwrap() as usize)));
    }

    // Multiplication
    pub fn mul(&mut self, instruction: Instr) {

        self.regs.set_reg(instruction.a as usize, self.regs.get_reg(instruction.b.unwrap() as usize).wrapping_mul(self.regs.get_reg(instruction.c.unwrap() as usize)));
    }

    // Division
    pub fn div(&mut self, instruction: Instr) {

        self.regs.set_reg(instruction.a as usize, self.regs.get_reg(instruction.b.unwrap() as usize).wrapping_div(self.regs.get_reg(instruction.c.unwrap() as usize)));
    }

    // Bitwise NAND
    pub fn nand(&mut self, instruction: Instr) {

        self.regs.set_reg(instruction.a as usize, !(self.regs.get_reg(instruction.b.unwrap() as usize) & self.regs.get_reg(instruction.c.unwrap() as usize)));
    }

    // Map Segment
    pub fn map_segment(&mut self, instruction: Instr) {

        self.regs.set_reg(instruction.b.unwrap() as usize, self.seg.alloc(self.regs.get_reg(instruction.c.unwrap() as usize) as usize) as u32);
    }

    // Unmap Segment
    pub fn unmap_segment(&mut self, instruction: Instr) {

        self.seg.remove(self.regs.get_reg(instruction.c.unwrap() as usize) as usize);
    }

    // Output
    pub fn output(&self, instruction: Instr) {
        
        stdout().write(&[self.regs.get_reg(instruction.c.unwrap() as usize) as u8]).unwrap();
    }

    // Input
    pub fn input(&mut self, instruction: Instr) {

        match stdin().bytes().next().unwrap() { 
            Ok(value) => {
                if value as char == '\n' {
                    self.regs.set_reg(instruction.c.unwrap() as usize, std::u32::MAX);
                } else {
                    self.regs.set_reg(instruction.c.unwrap() as usize, value as u32);
                }
            },
            Err(e) => panic!("Error reading input")
        }
    }

    // Load Program
    pub fn load_program(&mut self, instruction: Instr) -> usize {

        if self.regs.get_reg(instruction.b.unwrap() as usize) as usize != P_ADD {
            self.seg.load(self.regs.get_reg(instruction.b.unwrap() as usize) as usize);
        }

        self.regs.get_reg(instruction.c.unwrap() as usize) as usize
    }

    // Load Value
    pub fn load_value(&mut self, instruction: Instr) {

        self.regs.set_reg(instruction.a as usize, instruction.value.unwrap());
    }
}
