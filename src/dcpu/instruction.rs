use std::fmt::{self, Formatter, Display};

#[derive(Debug)]
pub enum Instruction {
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

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", *self)
    }
}

pub struct InstructionFactory;

impl InstructionFactory {
    pub fn new(word: &u16) -> Instruction {
        let special = InstructionFactory::is_special(word);
        let bits = match special {
            false => InstructionFactory::get_bits_in_range(word, 0, 5) as u8,
            true => InstructionFactory::get_bits_in_range(word, 5, 5) as u8
        };

        let a = InstructionFactory::a(word);
        let b = InstructionFactory::b(word);

        match (special, bits) {
            (false, 0x1) => Instruction::SET(a, b),
            (false, 0x2) => Instruction::ADD(a, b),
            (false, 0x3) => Instruction::SUB(a, b),
            (false, 0x4) => Instruction::MUL(a, b),
            (false, 0x5) => Instruction::MLI(a, b),
            (false, 0x6) => Instruction::DIV(a, b),
            (false, 0x7) => Instruction::DVI(a, b),
            (false, 0x8) => Instruction::MOD(a, b),
            (false, 0x9) => Instruction::MDI(a, b),
            (false, 0xa) => Instruction::AND(a, b),
            (false, 0xb) => Instruction::BOR(a, b),
            (false, 0xc) => Instruction::XOR(a, b),
            (false, 0xd) => Instruction::SHR(a, b),
            (false, 0xe) => Instruction::ASR(a, b),
            (false, 0xf) => Instruction::SHL(a, b),
            (false, 0x10) => Instruction::IFB(a, b),
            (false, 0x11) => Instruction::IFC(a, b),
            (false, 0x12) => Instruction::IFE(a, b),
            (false, 0x13) => Instruction::IFN(a, b),
            (false, 0x14) => Instruction::IFG(a, b),
            (false, 0x15) => Instruction::IFA(a, b),
            (false, 0x16) => Instruction::IFL(a, b),
            (false, 0x17) => Instruction::IFU(a, b),
            (false, 0x1a) => Instruction::ADX(a, b),
            (false, 0x1b) => Instruction::SBX(a, b),
            (false, 0x1e) => Instruction::STI(a, b),
            (false, 0x1f) => Instruction::STD(a, b),
            (true,  0x1) => Instruction::JSR(a),
            (true, 0x8) => Instruction::INT(a),
            (true, 0x9) => Instruction::IAG(a),
            (true, 0xa) => Instruction::IAS(a),
            (true, 0xb) => Instruction::RFI(a),
            _ => Instruction::NULL
        }
    }

    fn is_special(word: &u16) -> bool {
        InstructionFactory::get_bits_in_range(word, 0, 5) == 0
    }

    fn a(word: &u16) -> u8 {
        InstructionFactory::get_bits_in_range(word, 10, 6) as u8
    }

    fn b(word: &u16) -> u8 {
        InstructionFactory::get_bits_in_range(word, 5, 5) as u8
    }

    /// check if nth bit is written and rewrite it to result
    /// start is value in range 0 to 15
    fn get_bits_in_range(word: &u16, start: u8, length: u8) -> u16 {
        let mut result: u16 = 0;
        for n in start..(start+length) {
            result |= ((word >> n) & 1) << (n - start);
        }
        result
    }
}

