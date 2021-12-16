
const STRT_REG: usize = 3;
const STRT_VAL: usize = 25;
const STRT_OPC: usize = 4;

#[derive(Debug, PartialEq)]
pub enum Opcode {
    CMov,
    Load,
    Store,
    Add,
    Mul,
    Div,
    Nand,
    Halt,
    MapSegment,
    UnmapSegment,
    Output,
    Input,
    LoadProgram,
    LoadValue,
    Error 
}

// Splitting C part of the instruction
fn split_c(instruction: u32, opcode: &Opcode) -> Option<u32> {
    match *opcode {
        Opcode::LoadValue => None,
        _ => Some(
            {
                let bits = ((1 << STRT_REG) - 1);
                let temp: u32 = bits << 0;
                (instruction & temp) >> 0
            }
        )
    }
}

// Splitting B part of the instruction
fn split_b(instruction: u32, opcode: &Opcode) -> Option<u32> {
    match *opcode {
        Opcode::LoadValue => None,
        _ => Some(
            {
                let bits = ((1 << STRT_REG) - 1);
                let temp: u32 = bits << 3;
                (instruction & temp) >> 3
            }
        )
    }
}

// Splitting A part of the instruction
fn split_a(instruction: u32, opcode: &Opcode) -> u32 {
    match *opcode {
        Opcode::LoadValue => {
            let temp: u32 = ((1 << STRT_REG) - 1) << 25;
            (instruction & temp) >> 25
        },
        _ => 
            {
                let bits = ((1 << 3) - 1);
                let temp: u32 = bits << 6;
                (instruction & temp) >> 6
            }
    }
}


// Splitting value part of the instruction
fn split_value(instruction: u32, opcode: &Opcode) -> Option<u32> {
    match *opcode {
        Opcode::LoadValue => Some(
            {
            let bits = ((1 << STRT_VAL) - 1);
            let temp: u32 = bits << 0;
                (instruction & temp) >> 0
            }
        ),
        _ => None
    }
}

// Splitting opcode part of the instruction

fn split_opcode(instruction: u32) -> Opcode {
    let opcode = 
        {
            let bits = ((1 << STRT_OPC) - 1);
            let temp: u32 = bits << 28;
                (instruction & temp) >> 28
        };

    match opcode {
        0 => Opcode::CMov,
        1 => Opcode::Load,
        2 => Opcode::Store,
        3 => Opcode::Add,
        4 => Opcode::Mul,
        5 => Opcode::Div,
        6 => Opcode::Nand,
        7 => Opcode::Halt,
        8 => Opcode::MapSegment,
        9 => Opcode::UnmapSegment,
        10 => Opcode::Output,
        11 => Opcode::Input,
        12 => Opcode::LoadProgram,
        13 => Opcode::LoadValue,
        _ => Opcode::Error
    }
}


// Struct for an instruction
#[derive(Debug)]
pub struct Instr {
    pub opcode: Opcode,

    pub c: Option<u32>,
    pub b: Option<u32>,
    pub a: u32,
    
    pub value: Option<u32>,

    
}

impl Instr {
    pub fn new(instruction: u32) -> Instr {
        let opcode = split_opcode(instruction);

        
        let c = split_c(instruction, &opcode);
        let b = split_b(instruction, &opcode);
        let a = split_a(instruction, &opcode);

        let value = split_value(instruction, &opcode);
        

        Instr {
            opcode,
            c,
            b,
            a,
            value
        }
    }
}