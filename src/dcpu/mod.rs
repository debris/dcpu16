mod memory;
mod instruction;

use self::memory::Memory as Memory;
use self::instruction::Instruction as Instruction;
use self::instruction::Opcode as Opcode;

macro_rules! vec_with_size {
    ($size: expr) => ({
        let mut v = Vec::new(); 
        for i in 0..$size {
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

    fn process(&mut self) {
        while self.memory.is_readable(self.pc as usize) {
            let word = self.read_word();
            let i = Instruction(word);
            match i.opcode() {
                Opcode::SET=> {
                    let a = self.get_value(i.a());
                    self.set_value(i.b(), a);
                },
                Opcode::ADD => {
                    let a = self.get_value(i.a());
                    let b = self.get_value(i.b());
                    self.set_value(i.b(), a + b);
                },
                _ => () 
            }
        }
    }

    fn get_value(&mut self, addr: u8) -> Value {
        match addr {
            n @ 0x0 ... 0x7 => self.registers[n as usize],
            0x1b => self.sp,
            0x1c => self.pc,
            0x1d => self.ex,
            0x1f => self.read_word() as Value,
            n @ 0x20 ... 0x3f => (n - 0x21) as Value,
            _ => 0
        }
    }

    fn set_value(&mut self, addr: u8, value: Value) {
        match addr {
            n @ 0x0 ... 0x7 => self.registers[n as usize] = value,
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

