use crate::instr::Instruction;

pub struct RAM {
    data: Vec<u32>,
}

pub struct ROM {
    instructions: Vec<u32>
}

pub struct Memory {
    ram: RAM,
    rom: ROM,
}

impl Memory {
    pub fn new() -> Memory {
        Self {
            ram: RAM {
                data: vec![0; 2048],
            },
            rom: ROM {
                instructions: vec![],
            },
        }
    }

    pub fn read(&self, addr: u32) -> u32 {
        self.ram.data[addr as usize]
    }

    pub fn write(&mut self, addr: u32, data: u32) {
        self.ram.data[addr as usize] = data;
    }
}