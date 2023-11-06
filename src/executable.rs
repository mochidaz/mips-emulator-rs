use crate::instr::Instruction;

pub struct Executable {
	pub code: Vec<u32>,
	pub start_pc: u32
}

impl Executable {
	pub fn new() -> Executable {
		Self {
			code: vec![],
			start_pc: 0,
		}
	}

	pub fn write_word(&mut self, word: u32) {
		self.code.push(word);
	}

	pub fn assemble(&mut self, start_offset: u32, instructions: Vec<Instruction>) {
		for i in instructions {
			unimplemented!()
		}
	}
}