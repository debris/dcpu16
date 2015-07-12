mod memory;
mod instruction;

use self::memory::Memory as Memory;
use self::instruction::Instruction as Instruction;
use self::instruction::Opcode as Opcode;

macro_rules! vec_with_size {
    ($size: expr) => ({
        let mut v = Vec::new(); 
        for _i in 0..$size {
            v.push(0);
        }
        v
    })
}

type Value = u16;

pub struct Dcpu {
    memory: Memory,
    registers: Vec<Value>, // A - J
    cyc: Value,     // cycle
    pc: Value,      // program_counter
    sp: Value,      // stack_pointer
    ex: Value,      // extra
    ia: Value       // interupt address
}

impl Default for Dcpu {
    fn default() -> Dcpu {
        Dcpu {
            memory: Default::default(),
            registers: vec_with_size!(8),
            cyc: 0,
            pc: 0,
            sp: 0,
            ex: 0,
            ia: 0
        }
    }
}

impl Dcpu {
    fn load(&mut self, words: &[u16]) {
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
        self.sp += 1;
        res
    }

    fn process(&mut self) {
        while self.memory.is_readable(self.pc as usize) {
            let word = self.read_word();
            let i = Instruction(word);
            match i.opcode() {
                Opcode::SET => {
                    let a = self.get_value(i.a());
                    self.set_value(i.b(), a);
                },
                Opcode::ADD => {
                    let a = self.get_value(i.a());
                    let b = self.get_value(i.b());
                    self.set_value(i.b(), b + a);
                },
                Opcode::SUB => {
                    let a = self.get_value(i.a()); 
                    let b = self.get_value(i.b());
                    self.set_value(i.b(), b - a);
                },
                Opcode::JSR => {
                    let a = self.get_value(i.a());
                    let address = self.pc + 1;
                    self.push(address);
                    self.pc = a;
                },
                _ => () 
            }
        }
    }

    fn get_value(&mut self, addr: u8) -> Value {
        match addr {
            n @ 0x0 ... 0x7 => self.registers[n as usize],
            n @ 0x8 ... 0xf => self.memory.get(self.registers[(n - 0x8) as usize] as usize),
            0x18 => self.pop(),
            0x19 => self.memory.get(self.sp as usize), // peek
            0x1b => self.sp,
            0x1c => self.pc,
            0x1d => self.ex,
            0x1f => self.read_word() as Value,
            n @ 0x20 ... 0x3f => ((n as Value).wrapping_sub(0x21)),
            _ => 0
        }
    }

    fn set_value(&mut self, addr: u8, value: Value) {
        match addr {
            n @ 0x0 ... 0x7 => self.registers[n as usize] = value,
            0x18 => self.push(value),
            _ => ()
        }
    }
}

#[test]
fn test_set() {
    let mut cpu: Dcpu = Default::default();
    cpu.load(&[
             0xfc01,        // SET A, 30
             0x7c21, 0x001f // SET B, 31
    ]);
    cpu.process();
    assert_eq!(cpu.registers[0], 30);
    assert_eq!(cpu.registers[1], 31);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn test_add() {
    let mut cpu: Dcpu = Default::default();
    cpu.load(&[
             0xfc01,    // SET A, 30
             0x8821,    // SET B, 1
             0x0022     // ADD B, A
    ]);
    cpu.process(); 
    assert_eq!(cpu.registers[0], 30);
    assert_eq!(cpu.registers[1], 31);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn test_sub() {
    let mut cpu: Dcpu = Default::default();
    cpu.load(&[
             0x7c01, 0x0022,    // SET A, 34
             0x7c03, 0x001f     // SUB A, 31
    ]);
    cpu.process(); 
    assert_eq!(cpu.registers[0], 3);
    assert_eq!(cpu.pc, 4);
}

#[test]
fn test_push_pop_peek() {
    let mut cpu: Dcpu = Default::default();
    cpu.load(&[
             0x8f01,            // SET PUSH, 2
             0x7f01, 0x0023,    // SET PUSH, 35
             0x8b01,            // SET PUSH, 1
             0x6001,            // SET A, POP
             0x6002,            // ADD A, POP
             0x6421             // SET B, PEEK
    ]);
    cpu.process(); 
    assert_eq!(cpu.registers[0], 36);
    assert_eq!(cpu.registers[1], 2);
    assert_eq!(cpu.pc, 7);
    assert_eq!(cpu.sp, 0xffff);
}

#[test]
fn test_registers() {
    let mut cpu: Dcpu = Default::default();
    cpu.load(&[
             0xaf01,    // SET PUSH, 10
             0x8001,    // SET A, 0xffff
             0x2021     // SET B, [A]
    ]);
    cpu.process(); 

    assert_eq!(cpu.registers[0], 0xffff);
    assert_eq!(cpu.registers[1], 10);
    assert_eq!(cpu.sp, 0xffff);
    assert_eq!(cpu.pc, 3);
}


#[test]
fn test_jsr() {
    let mut cpu: Dcpu = Default::default();
    cpu.load(&[
             0x8801,    // SET A, 1
             0x8821,    // SET B, 1
             0x0422,    // ADD B, B
             0x0420     // JSR B
    ]);
    cpu.process(); 
    assert_eq!(cpu.registers[0], 1);
    assert_eq!(cpu.registers[1], 4);
    assert_eq!(cpu.sp, 0xfffe);
    assert_eq!(cpu.pc, 4);
}

