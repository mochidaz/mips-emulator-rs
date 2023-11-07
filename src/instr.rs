use crate::register::RegID;

pub enum AsmInstr {
	// Arithmetic
	Add(RegID, RegID, RegID),
    Addu(RegID, RegID, RegID),
    Addi(RegID, RegID, i32),
	Sub(RegID, RegID, RegID),
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

pub enum RtypeFn {
    Add = 0b100000,
    Addu = 0b100001,
    Sub = 0b100010,
    Subu = 0b100011,
}