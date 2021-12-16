mod seg;
mod reg;

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
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        if self.regs.get_reg(c) != 0 {
            self.regs.set_reg(a, self.regs.get_reg(b));
        }
    }

    // Segmented Load
    pub fn load(&mut self, instruction: Instr) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let array = self.seg.get_seg(self.regs.get_reg(b) as usize).unwrap();

        self.regs.set_reg(a, array[self.regs.get_reg(c) as usize]);
    }

    // Segmentted Store
    pub fn store(&mut self, instruction: Instr) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        self.seg.set_seg(self.regs.get_reg(a) as usize, self.regs.get_reg(b) as usize, self.regs.get_reg(c));
    }

    // Addition
    pub fn add(&mut self, instruction: Instr) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;


        self.regs.set_reg(a, self.regs.get_reg(b).wrapping_add(self.regs.get_reg(c)));
    }

    // Multiplication
    pub fn mul(&mut self, instruction: Instr) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        self.regs.set_reg(a, self.regs.get_reg(b).wrapping_mul(self.regs.get_reg(c)));
    }

    // Division
    pub fn div(&mut self, instruction: Instr) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        self.regs.set_reg(a, self.regs.get_reg(b).wrapping_div(self.regs.get_reg(c)));
    }

    // Bitwise NAND
    pub fn nand(&mut self, instruction: Instr) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        self.regs.set_reg(a, !(self.regs.get_reg(b) & self.regs.get_reg(c)));
    }

    // Map Segment
    pub fn map_segment(&mut self, instruction: Instr) {
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        self.regs.set_reg(b, self.seg.alloc(self.regs.get_reg(c) as usize) as u32);
    }

    // Unmap Segment
    pub fn unmap_segment(&mut self, instruction: Instr) {
        let c = instruction.c.unwrap() as usize;

        self.seg.remove(self.regs.get_reg(c) as usize);
    }

    // Output
    pub fn output(&self, instruction: Instr) {
        let c = instruction.c.unwrap() as usize;

        
        stdout().write(&[self.regs.get_reg(c) as u8]).unwrap();
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
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;


        if self.regs.get_reg(b) as usize != 0 {
            self.seg.load(self.regs.get_reg(b) as usize);
        }

        self.regs.get_reg(c) as usize
    }

    // Load Value
    pub fn load_value(&mut self, instruction: Instr) {
        let a = instruction.a as usize;

        self.regs.set_reg(a, instruction.value.unwrap());
    }
}
