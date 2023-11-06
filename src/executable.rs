use crate::instr::Instruction;

pub struct Executable {
	code: Vec<u32>,
	start_pc: u32
}

impl Executable {
	pub fn assemble(instructions: Vec<Instruction>) {
		for i in instructions {
			unimplemented!()
		}
	}
}