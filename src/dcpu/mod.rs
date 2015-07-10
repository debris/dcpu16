mod memory;
mod instruction;

use self::memory::Memory as Memory;
use self::instruction::Instruction as Instruction;

pub struct Dcpu {
    memory: Memory,
    cyc: u16,   // cycle
    pc: u16,    // program_counter
    sp: u16,    // stack_pointer
    ea: u16,    // extra_access
    ia: u16     // interupt address
}

impl Default for Dcpu {
    fn default() -> Dcpu {
        Dcpu {
            memory: Default::default(),
            cyc: 0,
            pc: 0,
            sp: 0,
            ea: 0,
            ia: 0
        }
    }
}

impl Dcpu {
    fn process(&mut self, word: u16) {
        let i = Instruction(word);
        if i.is_special() {
            self.process_special_instruction(&i);
        } else {
            self.process_instruction(&i);
        }
    }

    fn process_instruction(&mut self, i: &Instruction) {

    }

    fn process_special_instruction(&mut self, i: &Instruction) {
    }


}

