use crate::register::RegID;

pub enum Instruction {
	Add(RegID, RegID, RegID),
	Sub(RegID, RegID, RegID),
	Addi(RegID, RegID, i32),
	Addu(RegID, RegID, RegID),
	Subu(RegID, RegID, RegID),
	Mul(RegID, RegID, RegID),
	Mult(RegID, RegID),
	Div(RegID, RegID),
}
