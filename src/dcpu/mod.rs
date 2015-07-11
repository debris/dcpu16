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
    fn process(&mut self, word: u16) {
        let i = Instruction(word);
        self.pc += 1;
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

    fn get_value(&self, addr: u8) -> Value {
        match addr {
            n @ 0x0 ... 0x7 => self.registers[n as usize],
            0x1b => self.sp,
            0x1c => self.pc,
            0x1d => self.ex,
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
    cpu.process(0xfc01); // SET A, 30
    assert_eq!(cpu.registers[0], 30);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn test_add() {
    let mut cpu: Dcpu = Default::default();
    cpu.process(0xfc01); // SET A, 30
    cpu.process(0x8821); // SET B, 1
    cpu.process(0x0022); // ADD B, A
    assert_eq!(cpu.registers[0], 30);
    assert_eq!(cpu.registers[1], 31);
    assert_eq!(cpu.pc, 3);
}

