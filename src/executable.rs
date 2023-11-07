use crate::instr::AsmInstr;
use crate::register::RegID;

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

	pub fn assemble(&mut self, start_offset: u32, instructions: Vec<AsmInstr>) {
		for i in instructions {
			match i {
				AsmInstr::Add(dest, source, target) => {
					self.code.push(Self::encode_rtype(0u32, 0b100000u32, 0u32, dest, source, target));
				},
				AsmInstr::Sub(dest, source, target) => {
					self.code.push(Self::encode_rtype(0u32, 0b100010u32, 0u32, dest, source, target));
				},
				_ => todo!(),
			}
			unimplemented!()
		}
	}

	fn encode_rtype(opcode: u32, function: u32, sa: u32, dest: RegID, source: RegID, target: RegID) -> u32 {
		let rd = dest as u32;
		let rt = target as u32;
		let rs = source as u32;
		return (function & 0x3F) |
			((rd & 0x1F) << 11) |
			((rt & 0x1F) << 16) |
			((rs & 0x1F) << 21) |
			((opcode & 0x3F) << 26);
	}
}