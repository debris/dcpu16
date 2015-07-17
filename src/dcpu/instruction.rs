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
    SET(u8, u8),
    ADD(u8, u8),
    SUB(u8, u8),
    MUL(u8, u8),
    MLI(u8, u8),
    DIV(u8, u8),
    DVI(u8, u8),
    MOD(u8, u8),
    MDI(u8, u8),
    AND(u8, u8),
    BOR(u8, u8),
    XOR(u8, u8),
    SHR(u8, u8),
    ASR(u8, u8),
    SHL(u8, u8),
    IFB(u8, u8),
    IFC(u8, u8),
    IFE(u8, u8),
    IFN(u8, u8),
    IFG(u8, u8),
    IFA(u8, u8),
    IFL(u8, u8),
    IFU(u8, u8),
    ADX(u8, u8),
    SBX(u8, u8),
    STI(u8, u8),
    STD(u8, u8),

    JSR(u8),
    INT(u8),
    IAG(u8),
    IAS(u8),
    RFI(u8),

    NULL
}

pub struct Instruction(pub u16);

impl Instruction {
    pub fn opcode(&self) -> Opcode {
        let special = self.is_special();
        let bits = match special {
            false => get_bits_in_range(self.0, 0, 5) as u8,
            true => get_bits_in_range(self.0, 5, 5) as u8
        };
        match (special, bits) {
            (false, 0x1) => Opcode::SET(self.a(), self.b()),
            (false, 0x2) => Opcode::ADD(self.a(), self.b()),
            (false, 0x3) => Opcode::SUB(self.a(), self.b()),
            (false, 0x4) => Opcode::MUL(self.a(), self.b()),
            (false, 0x5) => Opcode::MLI(self.a(), self.b()),
            (false, 0x6) => Opcode::DIV(self.a(), self.b()),
            (false, 0x7) => Opcode::DVI(self.a(), self.b()),
            (false, 0x8) => Opcode::MOD(self.a(), self.b()),
            (false, 0x9) => Opcode::MDI(self.a(), self.b()),
            (false, 0xa) => Opcode::AND(self.a(), self.b()),
            (false, 0xb) => Opcode::BOR(self.a(), self.b()),
            (false, 0xc) => Opcode::XOR(self.a(), self.b()),
            (false, 0xd) => Opcode::SHR(self.a(), self.b()),
            (false, 0xe) => Opcode::ASR(self.a(), self.b()),
            (false, 0xf) => Opcode::SHL(self.a(), self.b()),
            (false, 0x10) => Opcode::IFB(self.a(), self.b()),
            (false, 0x11) => Opcode::IFC(self.a(), self.b()),
            (false, 0x12) => Opcode::IFE(self.a(), self.b()),
            (false, 0x13) => Opcode::IFN(self.a(), self.b()),
            (false, 0x14) => Opcode::IFG(self.a(), self.b()),
            (false, 0x15) => Opcode::IFA(self.a(), self.b()),
            (false, 0x16) => Opcode::IFL(self.a(), self.b()),
            (false, 0x17) => Opcode::IFU(self.a(), self.b()),
            (false, 0x1a) => Opcode::ADX(self.a(), self.b()),
            (false, 0x1b) => Opcode::SBX(self.a(), self.b()),
            (false, 0x1e) => Opcode::STI(self.a(), self.b()),
            (false, 0x1f) => Opcode::STD(self.a(), self.b()),
            (true,  0x1) => Opcode::JSR(self.a()),
            (true, 0x8) => Opcode::INT(self.a()),
            (true, 0x9) => Opcode::IAG(self.a()),
            (true, 0xa) => Opcode::IAS(self.a()),
            (true, 0xb) => Opcode::RFI(self.a()),
            _ => Opcode::NULL
        }
        //to_opcode(special, bits)
    }

    fn is_special(&self) -> bool {
        get_bits_in_range(self.0, 0, 5) == 0
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

