use crate::register::RegID;

pub enum Instruction {
	// Arithmetic
	Add(RegID, RegID, RegID),
	Sub(RegID, RegID, RegID),
	Addi(RegID, RegID, i32),
	Addu(RegID, RegID, RegID),
	Subu(RegID, RegID, RegID),
	Mul(RegID, RegID, RegID),
	Mult(RegID, RegID),
	Div(RegID, RegID),

	// Logical
	And(RegID, RegID, RegID),
	Andi(RegID, RegID, i32),
	Or(RegID, RegID, RegID),
	Ori(RegID, RegID, i32),
	Sll(RegID, RegID, i32),
	Srl(RegID, RegID, i32),

	// Data Transfer
	Lw(RegID, i32, RegID),
	Sw(RegID, i32, RegID),
	Lui(RegID, i32),
}