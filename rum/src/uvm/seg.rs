use std::mem;

use crate::startprogram::instr::Instr;

const P_ADD: usize = 0;

#[derive(Debug)]
pub struct Segment {
    allocated_heap: Vec<Vec<u32>>,

    list_allocator: Vec<usize>
}

impl Segment {
    pub fn new(instructions: Vec<u32>) -> Segment {
        Segment {
            list_allocator: Vec::new(),
            allocated_heap: vec![instructions]
        }
    }

    // Allocates memory with a vec of zeros witha given size and then returns the address of the new memory
    pub fn alloc(&mut self, size: usize) -> usize {
        let mut mem_address = P_ADD;

        if self.list_allocator.len() == 0 {
            self.allocated_heap.push(vec![0_u32; size]);

            self.allocated_heap.len() - 1
        } else {
            mem_address = self.list_allocator.pop().unwrap();

            mem::replace(
                self.allocated_heap.get_mut(mem_address)
                    .unwrap(),
                    vec![0_u32; size]
            );

            mem_address
        }

    }

    // Takes the given address and replaces the program with the vector at that address
    pub fn load(&mut self, address: usize) {
        let program = self.allocated_heap.get(address).unwrap().clone();

        mem::replace(
            self.allocated_heap.get_mut(P_ADD).unwrap(),
            program
        );
    }

    // Given the program vounter, takes the instruction corresponding to it
    pub fn get_instruction(&self, pc: usize) -> Instr{

        match self.allocated_heap.get(P_ADD) {
            Some(program) => Instr::new(program[pc]),
            None => panic!()
        }
    }

    // Puts the data of the memory in the address, but only if it's initialized
    pub fn get_seg(&self, address: usize) -> Option<&Vec<u32>> {
        self.allocated_heap.get(address)
    }

    // Takes the array at the given array index from the address and writes a value into it
    pub fn set_seg(&mut self, address: usize, index: usize, value: u32) {

        mem::replace(
            self.allocated_heap.get_mut(address).unwrap().get_mut(index).unwrap(),
            value
        );
    }

    // Removes the memory from the given address
    pub fn remove(&mut self, address: usize) {
        self.list_allocator.push(address);
        
        mem::replace(
            self.allocated_heap.get_mut(address).unwrap(),
            Vec::new()
        );
    }

    
}
