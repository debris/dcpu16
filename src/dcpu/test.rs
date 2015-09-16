use super::cpu::cpu::Cpu as Cpu;
use super::assembly::parser::Parser as Parser;

#[test]
fn test_set() {
    let mut parser = Parser::new("SET A, 30\n
                                  SET B, 1");
    let mut cpu = Cpu::new();
    cpu.load_program(&parser.parse());
    cpu.run();
    assert_eq!(cpu.A(), 30);
    assert_eq!(cpu.B(), 1);
}

#[test]
fn test_add() {
    let mut parser = Parser::new("SET A, 30\n
                                  SET B, 1\n
                                  ADD B, A");
    let mut cpu = Cpu::new();
    cpu.load_program(&parser.parse());
    cpu.run();
    assert_eq!(cpu.A(), 30);
    assert_eq!(cpu.B(), 31);
}

#[test]
fn test_mul() {
    let mut parser = Parser::new("SET A, 30\n
                                  SET B, 2\n
                                  MUL B, A");
    let mut cpu = Cpu::new();
    cpu.load_program(&parser.parse());
    cpu.run();
    assert_eq!(cpu.A(), 30);
    assert_eq!(cpu.B(), 60);
}

#[test]
fn test_div() {
    let mut parser = Parser::new("SET A, 5\n
                                  DIV A, 2");
    let mut cpu = Cpu::new();
    cpu.load_program(&parser.parse());
    cpu.run();
    assert_eq!(cpu.A(), 2);
}

#[test]
fn test_sti() {
    let mut parser = Parser::new("STI A, 15");
    let mut cpu = Cpu::new();
    cpu.load_program(&parser.parse());
    cpu.run();
    assert_eq!(cpu.A(), 15);
    assert_eq!(cpu.I(), 1);
    assert_eq!(cpu.J(), 1);
}

