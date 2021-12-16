// the regs module interface manages all of the 8 registers

const REG_LENGTH: usize = 8;

// struct represents that the UM consists of 8 unsigned 32-bit registers
#[derive(Debug)]
pub struct Regs {
    regs: [u32; REG_LENGTH]
}

impl Regs {
    // initializer funct
    pub fn new() -> Regs {
        Regs {
            regs: [0_u32; REG_LENGTH]
        }
    }

    // funct to get value at the register
    pub fn get_reg(&self, reg: usize) -> u32 {
        self.regs[reg]
    }

    // funct to store value at the register in a variable 
    pub fn set_reg(&mut self, reg: usize, value: u32) {
        self.regs[reg] = value
    }

}