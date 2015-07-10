
/// check if nth bit is written and rewrite it to result
/// start is value in range 0 to 15
fn get_bits_in_range(word: u16, start: u8, length: u8) -> u16 {
    let mut result: u16 = 0;
    for n in start..(start+length) {
        result |= (((word >> n) & 1) << (n - start));
    }
    result
}

pub struct Instruction(pub u16);

impl Instruction {
    pub fn is_special(&self) -> bool {
        self.opcode() == 0u8
    }

    pub fn opcode(&self) -> u8 {
        get_bits_in_range(self.0, 0, 5) as u8
    }

    pub fn special_opcode(&self) -> u8 {
        get_bits_in_range(self.0, 5, 5) as u8
    }

    pub fn a(&self) -> u8 {
        get_bits_in_range(self.0, 10, 6) as u8
    }

    pub fn b(&self) -> u8 {
        get_bits_in_range(self.0, 5, 5) as u8
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
fn special_opcode() {
    let i = Instruction(0b000011_00001_00000);
    assert_eq!(0b1u8, i.special_opcode());
}

#[test]
fn test_opcode() {
    let i = Instruction(0x7c01);
    let expected = 0b1u8; // SET
    assert_eq!(expected, i.opcode());
}

#[test]
fn test_a() {
    let i = Instruction(0x7c01);
    let expected = 0b11111u8; // 011111 = 1F = next word -> [PC++]
    assert_eq!(expected, i.a());
}

#[test]
fn test_b() {
    let i = Instruction(0x7c01);
    let expected = 0b0u8; // 00000 = register A
    assert_eq!(expected, i.b());
}

