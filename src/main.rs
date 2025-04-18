#![allow(dead_code)]

struct Cpu {
    acc: u8,         // 4 Bit accumulator
    regs: [u8; 16],  // 4 Bit registers 16 of them R0-R15
    pc: u16,         // 12 Bit PC
    sp: u16,         // 12 Bit SP
    flags: u8,       // 3 Flags (carry, zero, overflow)
    rom: [u8; 4096], // 4KB rom
}

impl Cpu {
    fn new(rom: [u8; 4096]) -> Self {
        Self {
            acc: 0,
            regs: [0; 16],
            pc: 0,
            sp: 0,
            flags: 0,
            rom,
        }
    }
    fn step(&mut self) {
        let instr = self.rom[self.pc as usize];
        let opr = instr >> 4;
        let opa = instr & 0x0F;
        self.pc += 1;
        match opr {
            0x0 => self.acc = self.regs[opa as usize],
            0x1 => self.acc = (self.acc + opa) & 0x0F,
            0xD => self.acc = opa,
            0xF => std::process::exit(0),
            _ => {}
        }
    }
}
fn main() {
    let mut rom = [0; 4096];
    rom[0] = 0xD5;
    rom[1] = 0x10;
    rom[2] = 0xF0;
    let mut cpu = Cpu::new(rom);
    loop {
        cpu.step();
    }
}
