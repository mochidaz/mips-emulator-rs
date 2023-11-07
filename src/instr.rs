use crate::register::RegID;

pub enum AsmInstr {
	Add(RegID, RegID, RegID),
    Addu(RegID, RegID, RegID),
    Addi(RegID, RegID, i32),
	Sub(RegID, RegID, RegID),
    Subu(RegID, RegID, RegID),
	Mul(RegID, RegID, RegID),
	Mult(RegID, RegID),
	Div(RegID, RegID),
}

pub enum RtypeFn {
    Add = 0b100000,
    Addu = 0b100001,
    Sub = 0b100010,
    Subu = 0b100011,
}