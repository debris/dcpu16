mod memory;
mod instruction;

use self::memory::Memory as Memory;
use self::instruction::Instruction as Instruction;
use self::instruction::Opcode as Opcode;

pub struct Cpu {
    memory: Memory,
    registers: [u16; 8], // A - J
    cyc: u16,     // cycle
    pc: u16,      // program_counter
    sp: u16,      // stack_pointer
    ex: u16,      // extra
    ia: u16       // interupt address
}

impl Default for Cpu {
    fn default() -> Cpu {
        Cpu {
            memory: Default::default(),
            registers: [0; 8],
            cyc: 0,
            pc: 0,
            sp: 0,
            ex: 0,
            ia: 0
        }
    }
}

impl Cpu {
    pub fn load_program(&mut self, words: &[u16]) {
        self.memory.load(words);
    }

    fn read_word(&mut self) -> u16 {
        let res = self.memory.get(self.pc as usize);
        self.pc += 1;
        res
    }

    fn push(&mut self, word: u16) {
        self.sp = self.sp.wrapping_sub(1);    
        self.memory.set(self.sp as usize, word);
    }

    fn pop(&mut self) -> u16 {
        let res = self.memory.get(self.sp as usize); 
        self.sp = self.sp.wrapping_add(1);
        res
    }

    pub fn run(&mut self) {
        while self.memory.has_word_at(self.pc as usize) {
            self.run_step();
        }
    }

    pub fn run_step(&mut self) {
        let word = self.read_word();
        match Instruction(word).opcode() {
            Opcode::SET(a, b) => {
                let va = self.get_value(a);
                self.set_value(b, va);
            },
            Opcode::ADD(a, b) => {
                let va = self.get_value(a) as u32;
                let vb = self.get_value(b) as u32;
                let res = vb + va;
                self.set_value(b, res as u16);
                self.ex = match res > 0xffff {
                    true => 0x1,
                    false => 0x0
                };
            },
            Opcode::SUB(a, b) => {
                let va = self.get_value(a) as i32;
                let vb = self.get_value(b) as i32;
                let res = vb - va;
                self.set_value(b, res as u16);
                self.ex = match res < 0 {
                    true => 0xffff,
                    false => 0x0
                };
            },
            Opcode::MUL(a, b) => {
                let va = self.get_value(a) as u32;
                let vb = self.get_value(b) as u32;
                let res = vb * va;
                self.set_value(b, res as u16);
                self.ex = ((res >> 16) & 0xffff) as u16;
            },
            Opcode::MLI(a, b) => {
                let va = self.get_value(a) as i16 as i32;
                let vb = self.get_value(b) as i16 as i32;
                let res = vb * va;
                self.set_value(b, res as u16);
                self.ex = ((res >> 16) & 0xffff) as u16;
            }, 
            Opcode::DIV(a, b) => {
                let va = self.get_value(a) as u32;
                match va {
                    0 => {
                        self.set_value(b, 0);
                        self.ex = 0;
                    },
                    _ => {
                        let vb = self.get_value(b) as u32;
                        let res = vb / va;
                        self.set_value(b, res as u16);
                        self.ex = (((vb << 16) / va) & 0xffff) as u16;
                    }
                };
            },
            Opcode::DVI(a, b) => {
                let va = self.get_value(a) as i16 as i32;
                match va {
                    0 => {
                        self.set_value(b, 0);
                        self.ex = 0;
                    },
                    _ => {
                        let vb = self.get_value(b) as i16 as i32;
                        let res = vb / va;
                        println!("a: {}, b: {}, res: {}", va, vb, res);
                        self.set_value(b, res as u16);
                        self.ex = (((vb << 16) / va) & 0xffff) as u16;
                    }
                };
            },
            Opcode::MOD(a, b) => {
                let va = self.get_value(a);
                let vb = self.get_value(b);
                match va {
                    0 => self.set_value(b, 0),
                    _ => self.set_value(b, vb % va)
                };
            },
            Opcode::MDI(a, b) => {
                let va = self.get_value(a) as i16;
                let vb = self.get_value(b) as i16;
                match va {
                    0 => self.set_value(b, 0),
                    _ => self.set_value(b, (vb % va) as u16)
                };
            },
            Opcode::AND(a, b) => {
                let va = self.get_value(a);
                let vb = self.get_value(b);
                self.set_value(b, vb & va); 
            },
            Opcode::BOR(a, b) => {
                let va = self.get_value(a);
                let vb = self.get_value(b);
                self.set_value(b, vb | va); 
            },
            Opcode::XOR(a, b) => {
                let va = self.get_value(a);
                let vb = self.get_value(b);
                self.set_value(b, vb ^ va); 
            },
            Opcode::SHR(a, b) => {
                let va = self.get_value(a) as u32;
                let vb = self.get_value(b) as u32;
                let res = vb >> va;
                self.set_value(b, res as u16); 
                self.ex = (((vb << 16) >> va) & 0xffff) as u16;
            },
            Opcode::ASR(a, b) => {
                let va = self.get_value(a) as i16 as i32;
                let vb = self.get_value(b) as i16 as i32;
                let res = vb >> va;
                self.set_value(b, res as u16); 
                self.ex = (((vb << 16) >> va) & 0xffff) as u16;
            },
            Opcode::SHL(a, b) => {
                let va = self.get_value(a) as u32;
                let vb = self.get_value(b) as u32;
                let res = vb << va;
                self.set_value(b, res as u16); 
                self.ex = (((vb << va) >> 16) & 0xffff) as u16;
            },
            Opcode::IFB(a, b) => {
                let va = self.get_value(a);
                let vb = self.get_value(b);
                if (vb & va) == 0 {
                    self.pc = self.pc + 1;
                }
            },
            Opcode::IFC(a, b) => {
                let va = self.get_value(a);
                let vb = self.get_value(b);
                if (vb & va) != 0 {
                    self.pc = self.pc + 1;
                }
            },
            Opcode::IFE(a, b) => {
                let va = self.get_value(a);
                let vb = self.get_value(b);
                if vb != va {
                    self.pc = self.pc + 1;
                }
            },
            Opcode::IFN(a, b) => {
                let va = self.get_value(a);
                let vb = self.get_value(b);
                if vb == va {
                    self.pc = self.pc + 1;
                }
            },
            Opcode::IFG(a, b) => {
                let va = self.get_value(a);
                let vb = self.get_value(b);
                if !(vb > va) {
                    self.pc = self.pc + 1;
                }
            },
            Opcode::IFA(a, b) => {
                let va = self.get_value(a) as i16;
                let vb = self.get_value(b) as i16;
                if !(vb > va) {
                    self.pc = self.pc + 1;
                }
            },
            Opcode::IFL(a, b) => {
                let va = self.get_value(a);
                let vb = self.get_value(b);
                if !(vb < va) {
                    self.pc = self.pc + 1;
                }
            },
            Opcode::IFU(a, b) => {
                let va = self.get_value(a) as i16;
                let vb = self.get_value(b) as i16;
                if !(vb < va) {
                    self.pc = self.pc + 1;
                }
            },
            Opcode::ADX(a, b) => {
                let va = self.get_value(a) as u32;
                let vb = self.get_value(b) as u32;
                let ex = self.ex as u32;
                let res = vb + va + ex;
                self.set_value(b, res as u16);
                self.ex = match res > 0xffff {
                    true => 0x1,
                    false => 0x0
                };
            },
            Opcode::SBX(a, b) => {
                let va = self.get_value(a) as i32;
                let vb = self.get_value(b) as i32;
                let ex = self.ex as i32;
                let res = vb - va + ex;
                self.set_value(b, res as u16);
                self.ex = match res < 0 {
                    true => 0xffff,
                    false => 0x0
                };
            },
            Opcode::STI(a, b) => {
                let va = self.get_value(a);
                self.set_value(b, va);
                self.registers[6] = self.registers[6].wrapping_add(1);
                self.registers[7] = self.registers[7].wrapping_add(1);
            },
            Opcode::STD(a, b) => {
                let va = self.get_value(a);
                self.set_value(b, va);
                self.registers[6] = self.registers[6].wrapping_sub(1);
                self.registers[7] = self.registers[7].wrapping_sub(1);
            },
            Opcode::JSR(a) => {
                let va = self.get_value(a);
                let address = self.pc + 1;
                self.push(address);
                self.pc = va;
            },
            Opcode::INT(a) if self.ia != 0 => {
                let va = self.get_value(a);
                let pc = self.pc;
                self.push(pc);
                let reg_a = self.registers[0];
                self.push(reg_a);
                self.pc = self.ia;
                self.registers[0] = va;
            },
            Opcode::INT(_) => {},
            Opcode::IAG(a) => {
                let ia = self.ia;
                self.set_value(a, ia);
            },
            Opcode::IAS(a) => {
                let va = self.get_value(a);
                self.ia = va;
            },
            Opcode::RFI(a) => {
                let va = self.get_value(a);
                self.registers[0] = self.pop();
                self.pc = self.pop();

            },
            _ => panic!()
        }
    }

    fn get_value(&mut self, addr: u8) -> u16 {
        match addr {
            n @ 0x0 ... 0x7 => self.registers[n as usize],
            n @ 0x8 ... 0xf => self.memory.get(self.registers[(n - 0x8) as usize] as usize),
            n @ 0x10 ... 0x17 => {
                let v = self.memory.get(self.registers[(n - 0x10) as usize] as usize);
                let word = self.read_word();
                self.memory.get(v.wrapping_add(word) as usize)
            },
            0x18 => self.pop(),
            0x19 => self.memory.get(self.sp as usize), // peek
            0x1a => {
                let sp = self.sp;
                let word = self.read_word();
                self.memory.get(sp.wrapping_add(word) as usize)
            },
            0x1b => self.sp,
            0x1c => self.pc,
            0x1d => self.ex,
            0x1e => {                                   // TODO: test
                let word = self.read_word() as usize;
                self.memory.get(word)
            },
            0x1f => self.read_word(),
            n @ 0x20 ... 0x3f => ((n as u16).wrapping_sub(0x21)),
            _ => panic!()
        }
    }

    fn set_value(&mut self, addr: u8, value: u16) {
        match addr {
            n @ 0x0 ... 0x7 => self.registers[n as usize] = value,
            0x18 => self.push(value),
            _ => panic!()
        }
    }
}

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

