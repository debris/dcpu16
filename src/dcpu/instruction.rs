use std::fmt::{self, Formatter, Display};

/// check if nth bit is written and rewrite it to result
/// start is value in range 0 to 15
fn get_bits_in_range(word: u16, start: u8, length: u8) -> u16 {
    let mut result: u16 = 0;
    for n in start..(start+length) {
        result |= ((word >> n) & 1) << (n - start);
    }
    result
}

#[derive(Debug)]
pub enum Opcode {
    SET,
    ADD,
    SUB,
    MUL,
    MLI,
    DIV,
    DVI,
    MOD,
    MDI,
    AND,
    BOR,
    XOR,
    SHR,
    ASR,
    SHL,
    IFB,
    IFC,

    JSR,

    NULL
}

fn to_opcode(special: bool, bits: u8) -> Opcode {
    match (special, bits) {
        (false, 0x1) => Opcode::SET,
        (false, 0x2) => Opcode::ADD,
        (false, 0x3) => Opcode::SUB,
        (false, 0x4) => Opcode::MUL,
        (false, 0x5) => Opcode::MLI,
        (false, 0x6) => Opcode::DIV,
        (false, 0x7) => Opcode::DVI,
        (false, 0x8) => Opcode::MOD,
        (false, 0x9) => Opcode::MDI,
        (false, 0xa) => Opcode::AND,
        (false, 0xb) => Opcode::BOR,
        (false, 0xc) => Opcode::XOR,
        (false, 0xd) => Opcode::SHR,
        (false, 0xe) => Opcode::ASR,
        (false, 0xf) => Opcode::SHL,
        (false, 0x10) => Opcode::IFB,
        (false, 0x11) => Opcode::IFC,
        (true,  0x1) => Opcode::JSR,
        _ => Opcode::NULL
    }
}

pub struct Instruction(pub u16);

impl Instruction {
    pub fn is_special(&self) -> bool {
        get_bits_in_range(self.0, 0, 5) == 0
    }

    pub fn opcode(&self) -> Opcode {
        let special = self.is_special();
        let bits = match special {
            false => get_bits_in_range(self.0, 0, 5) as u8,
            true => get_bits_in_range(self.0, 5, 5) as u8
        };
        to_opcode(special, bits)
    }

    pub fn a(&self) -> u8 {
        get_bits_in_range(self.0, 10, 6) as u8
    }

    pub fn b(&self) -> u8 {
        match self.is_special() {
            false => get_bits_in_range(self.0, 5, 5) as u8,
            true => 0 // returning 0 is safer than returning some random opcode
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.is_special() {
            true => write!(f, "special, op: {:?}, a: 0x{:x}", self.opcode(), self.a()),
            false => write!(f, "normal, op: {:?}, a: 0x{:x}, b: 0x{:x}", self.opcode(), self.a(), self.b())
        }
    }
}

#[test]
fn test_is_special() {
    let i = Instruction(0b000011_00001_00000);
    let i2 = Instruction(0b000011_00001_00001);
    assert_eq!(true, i.is_special());
    assert_eq!(false, i2.is_special());
}

#[test]
fn test_opcode() {
    //let i = Instruction(0x7c01);
    //let i2 = Instruction(0b000011_00001_00000);
    //assert_eq!(Opcode::SET, i.opcode());
    //assert_eq!(Opcode::SET, i2.opcode());
}

#[test]
fn test_a() {
    let i = Instruction(0x7c01);
    let i2 = Instruction(0b111111_00000_00001);
    let i3 = Instruction(0xfc01);
    assert_eq!(0b11111, i.a()); // 011111 = 1F = next word -> [PC++]
    assert_eq!(0b111111, i2.a()); // 111111 = 
    assert_eq!(0b111111, i3.a()); // 111111 = 
}

#[test]
fn test_b() {
    let i = Instruction(0x7c01);
    let expected = 0b0u8; // 00000 = register A
    assert_eq!(expected, i.b());
}

