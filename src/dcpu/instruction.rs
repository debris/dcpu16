
// check if nth bit is written and rewrite it to result
fn getBitsInRange(word: u16, start: u8, length: u8) -> u16 {
    let mut result: u16 = 0;
    for n in start..(start+length) {
        result |= (((word >> n) & 1) << (n - start));
    }
    result
}

struct Instruction(u16);

impl Instruction {
    fn opcode(&self) -> u8 {
        getBitsInRange(self.0, 0, 5) as u8
    }

    fn a(&self) -> u8 {
        getBitsInRange(self.0, 10, 6) as u8
    }

    fn b(&self) -> u8 {
        getBitsInRange(self.0, 5, 5) as u8
    }
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

