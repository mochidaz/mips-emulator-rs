pub struct RAM {
    data: Vec<u32>,
}

pub struct ROM {
}

pub struct Memory {
    ram: RAM,
    rom: ROM,
}