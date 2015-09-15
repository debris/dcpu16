use super::cpu::cpu::Cpu as Cpu;
use super::assembly::tokenizer::Parser as Parser;

#[test]
fn test_dcpu() {
    let mut parser = Parser::new("SET A, 30\n
                                  SET B, 1");
    let mut cpu = Cpu::new();
    cpu.load_program(&parser.parse());
    cpu.run();
    assert_eq!(cpu.registers()[0], 30);
    assert_eq!(cpu.registers()[1], 1);
}

