use crate::register::RegID;

pub enum AsmInstr {
	// Arithmetic
	Add(RegID, RegID, RegID),
    Addu(RegID, RegID, RegID),
    Addi(RegID, RegID, i32),
	Sub(RegID, RegID, RegID),
    Subu(RegID, RegID, RegID),
	Mult(RegID, RegID),
    Multu(RegID, RegID),
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
	Mult = 0b011000,
	Div = 0b011010,
	And = 0b100100,
	Or = 0b100101,
}

pub enum OpCode {
    Rtype = 0,
    Addi = 0b001000,
    Addiu = 0b001001,
    Andi = 0b001100,
    Ori = 0b001101,
    Lui = 0b001111,
    Lb = 0b100000,
    Lbu = 0b100100,
    Lh = 0b100001,
    Lhu = 0b100101,
	Lw = 0b100011,
    Sb = 0b101000,
    Sh = 0b101001,
	Sw = 0b101011,
}