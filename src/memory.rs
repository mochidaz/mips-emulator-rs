use crate::instr::Instruction;
use crate::executable::Executable;

pub struct Memory {
    ram: Vec<u32>,
    rom: Vec<u32>,
}

impl Memory {
    pub fn new() -> Memory {
        Self {
            ram: vec![0; 2048],
            rom: vec![],
        }
    }

    pub fn read(&self, addr: u32) -> u32 {
        self.ram[addr as usize]
    }

    pub fn write(&mut self, addr: u32, data: u32) {
        self.ram[addr as usize] = data;
    }

    pub fn write_rom(&mut self, executable: &Executable) {
        self.rom = executable.code.clone();
    }
}