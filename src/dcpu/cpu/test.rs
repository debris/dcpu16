use super::cpu::Cpu as Cpu;

#[test]
fn test_set() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0xfc01,        // SET A, 30
             0x7c21, 0x001f // SET B, 31
    ]);
    cpu.run();
    assert_eq!(cpu.registers[0], 30);
    assert_eq!(cpu.registers[1], 31);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn test_add() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0xfc01,    // SET A, 30
             0x8821,    // SET B, 1
             0x0022     // ADD B, A
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 30);
    assert_eq!(cpu.registers[1], 31);
    assert_eq!(cpu.pc, 3);
    assert_eq!(cpu.ex, 0);
}

#[test]
fn test_add_overflow() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x8001,    // SET A, 0xffff
             0x8c02     // ADD A, 2
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 1);
    assert_eq!(cpu.pc, 2);
    assert_eq!(cpu.ex, 1);
}

#[test]
fn test_sub() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x7c01, 0x0022,    // SET A, 34
             0x7c03, 0x001f     // SUB A, 31
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 3);
    assert_eq!(cpu.pc, 4);
}

#[test]
fn test_sub_underflow() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x8c01,    // SET A, 2
             0x9403     // SUB A, 4
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 0xfffe);
    assert_eq!(cpu.pc, 2);
    assert_eq!(cpu.ex, 0xffff);
}

#[test]
fn test_push_pop_peek() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x8f01,            // SET PUSH, 2
             0x7f01, 0x0023,    // SET PUSH, 35
             0x8b01,            // SET PUSH, 1
             0x6001,            // SET A, POP
             0x6002,            // ADD A, POP
             0x6421             // SET B, PEEK
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 36);
    assert_eq!(cpu.registers[1], 2);
    assert_eq!(cpu.pc, 7);
    assert_eq!(cpu.sp, 0xffff);
}

#[test]
fn test_registers() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0xaf01,    // SET PUSH, 10
             0x8001,    // SET A, 0xffff
             0x2021     // SET B, [A]
    ]);
    cpu.run(); 

    assert_eq!(cpu.memory.get(0xffff), 10);
    assert_eq!(cpu.registers[0], 0xffff);
    assert_eq!(cpu.registers[1], 10);
    assert_eq!(cpu.sp, 0xffff);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn test_jsr() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x8801,    // SET A, 1
             0x8821,    // SET B, 1
             0x0422,    // ADD B, B
             0x0420     // JSR B
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 1);
    assert_eq!(cpu.registers[1], 4);
    assert_eq!(cpu.sp, 0xfffe);
    assert_eq!(cpu.pc, 4);
}

#[test]
fn test_mul() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0xc001,    // SET A, 15
             0x8c04     // MUL A, 2
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 30);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn test_mul_overflow() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x9401,    // SET A, 4
             0x8004     // MUL A, 0xffff
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 0xfffc);
    assert_eq!(cpu.pc, 2);
    assert_eq!(cpu.ex, 3);
}

#[test]
fn test_mli() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0xc001,    // SET A, 15
             0x8c05     // MLI A, 2
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 30);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn test_mli_overflow() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x9401,    // SET A, 4
             0x8005     // MLI A, 0xffff
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 0xfffc);
    assert_eq!(cpu.pc, 2);
    assert_eq!(cpu.ex, 0xffff); 
}

#[test]
fn test_div() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x9801,    // SET A, 5
             0x8c06     // DIV A, 2
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 2);
    assert_eq!(cpu.pc, 2);
    assert_eq!(cpu.ex, 0x8000);
}

#[test]
fn test_dvi() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x9801,    // SET A, 5
             0x8c07     // DVI A, 2
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 2);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn test_dvi_signed() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x9801,            // SET A, 5
             0x7c07, 0xfffe     // DVI A, -2
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 0xfffe);
    assert_eq!(cpu.pc, 3);
    assert_eq!(cpu.ex, 0x8000);
}

#[test]
fn test_mod() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x9801,    // SET A, 5
             0x8c08     // MOD A, 2
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 1);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn test_mdi() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x9801,    // SET A, 5
             0x8c09     // MDI A, 2
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 1);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn test_mdi_signed() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x7c01, 0xfff9,    // SET A, -7
             0xc409             // MDI A, 16
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 0xfff9);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn test_and() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0xa001,    // SET A, 7
             0x980a     // AND A, 5
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 5);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn test_bor() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x9001,    // SET A, 3
             0x980b     // BOR A, 5
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 7);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn test_xor() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x9001,    // SET A, 3
             0x980c     // XOR A, 5
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 6);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn test_shr() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0xa001,    // SET A, 7
             0x880d     // SHR A, 1
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 3);
    assert_eq!(cpu.pc, 2);
    assert_eq!(cpu.ex, 0x8000);
}

#[test]
fn test_asr() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0xa001,    // SET A, 7
             0x880e     // ASR A, 1
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 3);
    assert_eq!(cpu.pc, 2);
    assert_eq!(cpu.ex, 0x8000);
}

#[test]
fn test_shl() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0xa001,    // SET A, 7
             0x880f     // SHL A, 1
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 14);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn test_shl_ex() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0xa001,    // SET A, 7
             0xe40f     // SHL A, 24
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 0);
    assert_eq!(cpu.pc, 2);
    assert_eq!(cpu.ex, 0x0700);
}

#[test]
fn test_ifb() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x8801,    // SET A, 1
             0x9010,    // IFB A, 3
             0x8c01,    // SET A, 2
             0x8810,    // IFB A, 1
             0x8821     // SET B, 1
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 2);
    assert_eq!(cpu.registers[1], 0);
    assert_eq!(cpu.pc, 5);
}

#[test]
fn test_ifc() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x8801,    // SET A, 1
             0x9011,    // IFC A, 3
             0x8c01,    // SET A, 2
             0x8c11,    // IFC A, 2
             0x8821     // SET B, 1
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 1);
    assert_eq!(cpu.registers[1], 1);
    assert_eq!(cpu.pc, 5);
}

#[test]
fn test_ife() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x8801,    // SET A, 1
             0x8c12,    // IFE A, 2
             0x8c01,    // SET A, 2
             0x8812,    // IFE A, 1
             0x8821     // SET B, 1
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 1);
    assert_eq!(cpu.registers[1], 1);
    assert_eq!(cpu.pc, 5);
}

#[test]
fn test_ifn() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x8801,    // SET A, 1
             0x8c13,    // IFN A, 2
             0x8c01,    // SET A, 2
             0x8c13,    // IFN A, 2
             0x8821     // SET B, 1
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 2);
    assert_eq!(cpu.registers[1], 0);
    assert_eq!(cpu.pc, 5);
}

#[test]
fn test_ifg() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x8c01,    // SET A, 2
             0x8814,    // IFG A, 1
             0x8801,    // SET A, 1
             0x8814,    // IFG A, 1
             0x8821     // SET B, 1
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 1);
    assert_eq!(cpu.registers[1], 0);
    assert_eq!(cpu.pc, 5);
}

#[test]
fn test_ifa() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x8c01,    // SET A, 2
             0x8815,    // IFA A, 1
             0x8801,    // SET A, 1
             0x8815,    // IFA A, 1
             0x8821     // SET B, 1
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 1);
    assert_eq!(cpu.registers[1], 0);
    assert_eq!(cpu.pc, 5);
}

#[test]
fn test_ifl() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x8801,    // SET A, 1
             0x8c16,    // IFL A, 2
             0x8c01,    // SET A, 2
             0x8c16,    // IFL A, 2
             0x8821     // SET B, 1
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 2);
    assert_eq!(cpu.registers[1], 0);
    assert_eq!(cpu.pc, 5);
}

#[test]
fn test_ifu() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x8801,    // SET A, 1
             0x8c17,    // IFU A, 2
             0x8c01,    // SET A, 2
             0x8c17,    // IFU A, 2
             0x8821     // SET B, 1
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 2);
    assert_eq!(cpu.registers[1], 0);
    assert_eq!(cpu.pc, 5);
}

#[test]
fn test_sti() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0xc01e     // STI A, 15
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 15);
    assert_eq!(cpu.registers[6], 1);
    assert_eq!(cpu.registers[7], 1);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn test_std() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0xc01f     // STD A, 15
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 15);
    assert_eq!(cpu.registers[6], 0xffff);
    assert_eq!(cpu.registers[7], 0xffff);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn test_int() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x9401,            // SET A, 4
             0x7d40, 0x0006,    // IAS 6
             0x7d00, 0x0008,    // INT 8
             0x9021,            // SET B, 3
             0xa041             // SET C, 7
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 8);
    assert_eq!(cpu.registers[1], 0);
    assert_eq!(cpu.registers[2], 7);
    assert_eq!(cpu.pc, 7);
    assert_eq!(cpu.sp, 0xfffe);
    assert_eq!(cpu.ia, 6);
}

#[test]
fn test_rfi() {
    let mut cpu: Cpu = Default::default();
    cpu.load_program(&[
             0x9401,            // SET A, 4
             0x7d40, 0x0006,    // IAS 6
             0x7d00, 0x0008,    // INT 8
             0x9021,            // SET B, 3
             0xa042,            // ADD C, 7
             0x7d60, 0x0001     // RFI 1
    ]);
    cpu.run(); 
    assert_eq!(cpu.registers[0], 0x9401);
    assert_eq!(cpu.registers[1], 3);
    assert_eq!(cpu.registers[2], 14);
    assert_eq!(cpu.pc, 0x7d40);
    assert_eq!(cpu.sp, 2);
    assert_eq!(cpu.ia, 6);
}

