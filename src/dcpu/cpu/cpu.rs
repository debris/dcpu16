use super::memory::Memory as Memory;
use super::instruction::InstructionFactory as InstructionFactory;
use super::instruction::Instruction as Instruction;

pub struct Cpu {
    pub memory: Memory,
    pub registers: [u16; 8], // A - J
    pub cyc: u16,     // cycle
    pub pc: u16,      // program_counter
    pub sp: u16,      // stack_pointer
    pub ex: u16,      // extra
    pub ia: u16       // interupt address
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
        match InstructionFactory::new(&word) {
            Instruction::SET(a, b) => {
                let va = self.get_value(a);
                self.set_value(b, va);
            },
            Instruction::ADD(a, b) => {
                let va = self.get_value(a) as u32;
                let vb = self.get_value(b) as u32;
                let res = vb + va;
                self.set_value(b, res as u16);
                self.ex = match res > 0xffff {
                    true => 0x1,
                    false => 0x0
                };
            },
            Instruction::SUB(a, b) => {
                let va = self.get_value(a) as i32;
                let vb = self.get_value(b) as i32;
                let res = vb - va;
                self.set_value(b, res as u16);
                self.ex = match res < 0 {
                    true => 0xffff,
                    false => 0x0
                };
            },
            Instruction::MUL(a, b) => {
                let va = self.get_value(a) as u32;
                let vb = self.get_value(b) as u32;
                let res = vb * va;
                self.set_value(b, res as u16);
                self.ex = ((res >> 16) & 0xffff) as u16;
            },
            Instruction::MLI(a, b) => {
                let va = self.get_value(a) as i16 as i32;
                let vb = self.get_value(b) as i16 as i32;
                let res = vb * va;
                self.set_value(b, res as u16);
                self.ex = ((res >> 16) & 0xffff) as u16;
            }, 
            Instruction::DIV(a, b) => {
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
            Instruction::DVI(a, b) => {
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
            Instruction::MOD(a, b) => {
                let va = self.get_value(a);
                let vb = self.get_value(b);
                match va {
                    0 => self.set_value(b, 0),
                    _ => self.set_value(b, vb % va)
                };
            },
            Instruction::MDI(a, b) => {
                let va = self.get_value(a) as i16;
                let vb = self.get_value(b) as i16;
                match va {
                    0 => self.set_value(b, 0),
                    _ => self.set_value(b, (vb % va) as u16)
                };
            },
            Instruction::AND(a, b) => {
                let va = self.get_value(a);
                let vb = self.get_value(b);
                self.set_value(b, vb & va); 
            },
            Instruction::BOR(a, b) => {
                let va = self.get_value(a);
                let vb = self.get_value(b);
                self.set_value(b, vb | va); 
            },
            Instruction::XOR(a, b) => {
                let va = self.get_value(a);
                let vb = self.get_value(b);
                self.set_value(b, vb ^ va); 
            },
            Instruction::SHR(a, b) => {
                let va = self.get_value(a) as u32;
                let vb = self.get_value(b) as u32;
                let res = vb >> va;
                self.set_value(b, res as u16); 
                self.ex = (((vb << 16) >> va) & 0xffff) as u16;
            },
            Instruction::ASR(a, b) => {
                let va = self.get_value(a) as i16 as i32;
                let vb = self.get_value(b) as i16 as i32;
                let res = vb >> va;
                self.set_value(b, res as u16); 
                self.ex = (((vb << 16) >> va) & 0xffff) as u16;
            },
            Instruction::SHL(a, b) => {
                let va = self.get_value(a) as u32;
                let vb = self.get_value(b) as u32;
                let res = vb << va;
                self.set_value(b, res as u16); 
                self.ex = (((vb << va) >> 16) & 0xffff) as u16;
            },
            Instruction::IFB(a, b) => {
                let va = self.get_value(a);
                let vb = self.get_value(b);
                if (vb & va) == 0 {
                    self.pc = self.pc + 1;
                }
            },
            Instruction::IFC(a, b) => {
                let va = self.get_value(a);
                let vb = self.get_value(b);
                if (vb & va) != 0 {
                    self.pc = self.pc + 1;
                }
            },
            Instruction::IFE(a, b) => {
                let va = self.get_value(a);
                let vb = self.get_value(b);
                if vb != va {
                    self.pc = self.pc + 1;
                }
            },
            Instruction::IFN(a, b) => {
                let va = self.get_value(a);
                let vb = self.get_value(b);
                if vb == va {
                    self.pc = self.pc + 1;
                }
            },
            Instruction::IFG(a, b) => {
                let va = self.get_value(a);
                let vb = self.get_value(b);
                if !(vb > va) {
                    self.pc = self.pc + 1;
                }
            },
            Instruction::IFA(a, b) => {
                let va = self.get_value(a) as i16;
                let vb = self.get_value(b) as i16;
                if !(vb > va) {
                    self.pc = self.pc + 1;
                }
            },
            Instruction::IFL(a, b) => {
                let va = self.get_value(a);
                let vb = self.get_value(b);
                if !(vb < va) {
                    self.pc = self.pc + 1;
                }
            },
            Instruction::IFU(a, b) => {
                let va = self.get_value(a) as i16;
                let vb = self.get_value(b) as i16;
                if !(vb < va) {
                    self.pc = self.pc + 1;
                }
            },
            Instruction::ADX(a, b) => {
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
            Instruction::SBX(a, b) => {
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
            Instruction::STI(a, b) => {
                let va = self.get_value(a);
                self.set_value(b, va);
                self.registers[6] = self.registers[6].wrapping_add(1);
                self.registers[7] = self.registers[7].wrapping_add(1);
            },
            Instruction::STD(a, b) => {
                let va = self.get_value(a);
                self.set_value(b, va);
                self.registers[6] = self.registers[6].wrapping_sub(1);
                self.registers[7] = self.registers[7].wrapping_sub(1);
            },
            Instruction::JSR(a) => {
                let va = self.get_value(a);
                let address = self.pc + 1;
                self.push(address);
                self.pc = va;
            },
            Instruction::INT(a) if self.ia != 0 => {
                let va = self.get_value(a);
                let pc = self.pc;
                self.push(pc);
                let reg_a = self.registers[0];
                self.push(reg_a);
                self.pc = self.ia;
                self.registers[0] = va;
            },
            Instruction::INT(_) => {},
            Instruction::IAG(a) => {
                let ia = self.ia;
                self.set_value(a, ia);
            },
            Instruction::IAS(a) => {
                let va = self.get_value(a);
                self.ia = va;
            },
            Instruction::RFI(a) => {
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

