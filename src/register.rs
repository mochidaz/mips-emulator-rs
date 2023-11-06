pub enum RegID {
    Zero,
    V0, V1,
    A0, A1, A2, A3,
    T0, T1, T2, T3,
    T4, T5, T6, T7,
    T8, T9,
    S0, S1, S2, S3,
    S4, S5, S6, S7,
}

pub trait Castable<T> {
    fn cast(&self) -> T;
}

pub struct Register {
    data: u32,
}

impl Register {
    pub fn new(data: u32) -> Register {
        Self {
            data
        }
    }
}

impl Castable<u8> for Register {
    fn cast(&self) -> u8 {
        self.data as u8
    }
}

impl Castable<u16> for Register {
    fn cast(&self) -> u16 {
        self.data as u16
    }
}

impl Castable<u64> for Register {
    fn cast(&self) -> u64 {
        self.data as u64
    }
}

impl Castable<i8> for Register {
    fn cast(&self) -> i8 {
        self.data as i8
    }
}

impl Castable<i16> for Register {
    fn cast(&self) -> i16 {
        self.data as i16
    }
}

impl Castable<i32> for Register {
    fn cast(&self) -> i32 {
        self.data as i32
    }
}

impl Castable<i64> for Register {
    fn cast(&self) -> i64 {
        self.data as i64
    }
}

